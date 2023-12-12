use std::collections::{HashMap, HashSet};
use std::env;
use actix_web::{HttpResponse, Responder, web};
use diesel::{QueryDsl, RunQueryDsl};
use rand::Rng;
use diesel::ExpressionMethods;
use reqwest::header;
use std::option::Option;
use serde::Deserialize;
use common::crypto::hash::sha256;
use common::util::auth::Identity;
use common::util::response::ApiResponse;

use crate::ConnPool;
use crate::schema::users::dsl::users;
use crate::schema::users::{password, username};
use crate::user::data_model::{Response, StudentDatum};
use crate::user::model::User;

#[derive(Debug, Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

pub async fn create_user(
    pool: web::Data<ConnPool>
) -> impl Responder{
    let mut map = HashMap::new();
    map.insert("page",1);
    map.insert("pageSize",5000);
    let cookie_string =
        format!(
            "{}={}",
            env::var("COOKIE_NAME").unwrap(),
            env::var("COOKIE_VALUE").unwrap());
    let mut request_headers = header::HeaderMap::new();
    request_headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&cookie_string).unwrap(),
    );
    let client = reqwest::ClientBuilder::new()
        .default_headers(request_headers)
        .build()
        .unwrap();
    let mut student_data = Vec::new();
    loop{
        let res = client
            .get(env::var("MANAGER_BASEURL")
                .unwrap()+"/api/studentData/")
            .query(&map)
            .send()
            .await
            .unwrap();
        let res_string = res.text().await.unwrap();
        let mut model: Response = serde_json::from_str(&res_string).unwrap();
        let num = model.data.student_data.len();
        student_data.extend(model.data.student_data.drain(..));
        if num >= 5000{
            continue;
        }
        break;
    }
    let mut conn = pool.get().unwrap();
    let user_vec = users.load::<User>(&mut conn).unwrap();
    let insert_data = find_extra_students(&student_data,&user_vec);
    let mut user_vec: Vec<User> = Vec::new();
    for item in insert_data.iter(){
        let s = generate_random_string(8);
        let new_user = User{
            id: None,
            username: Option::from(s.clone()),
            password: Option::from(sha256(s.clone())),
            name: Option::from(item.name.clone().unwrap()),
            student_id: Option::from(item.id.clone().unwrap()),
        };
        user_vec.push(new_user);
    }
    diesel::insert_into(users)
        .values(&user_vec)
        .execute(&mut conn)
        .expect("Error inserting new users");
    HttpResponse::Ok()
        .json(ApiResponse{
            code: 0,
            msg: "success".to_string(),
            data: insert_data,
        })
}

fn generate_random_string(length: usize) -> String {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| char::from(u8::try_from(charset[rng.gen_range(0..charset.len())] as char).unwrap()))
        .collect();
    random_string
}

fn find_extra_students<'a>(
    student_data: &'a [StudentDatum],
    us: &[User]) -> Vec<&'a StudentDatum> {
    // 构建 HashSet 以提高查找性能
    let user_set: HashSet<_> = us.iter()
        .map(|user| (user.name.as_deref(), user.student_id))
        .collect();

    // 使用 filter 方法筛选出多余的 StudentDatum 数据
    let extra_students: Vec<_> = student_data
        .iter()
        .filter(|student| {
            !user_set.contains(&(student.name.as_deref(), student.id))
        })
        .collect();

    extra_students
}




pub async fn login(
    login_request: web::Json<LoginRequest>,
    pool: web::Data<ConnPool>
) -> impl Responder{
    println!("{:?}", login_request);
    let login = login_request.into_inner();
    let mut conn = pool.get().unwrap();
    return match users
        .filter(username.eq(Option::from(login.username.clone())))
        .select(password).first::<Option<String>>(&mut conn) {
        Ok(p) => {
            if p.unwrap()== sha256(login.password) {
                let t = Identity::new(0,0,login.username.clone());
                let mut map_identity = HashMap::new();
                map_identity.insert("token", t.to_auth_token().unwrap());
                return HttpResponse::Ok().json(ApiResponse{
                    code: 0,
                    msg: "login success".to_string(),
                    data: map_identity,
                });
            }
            HttpResponse::Ok().json(ApiResponse{
                code: 1,
                msg: "login fail".to_string(),
                data: (),
            })
        }
        Err(_) => {
            HttpResponse::Ok().json(ApiResponse{
                code: 1,
                msg: "login fail".to_string(),
                data: (),
            })
        }
    };
}
