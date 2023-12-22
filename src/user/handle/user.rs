use std::collections::{HashMap, HashSet};
use std::env;
use actix_web::{HttpResponse, Responder, web};
use diesel::{JoinOnDsl, QueryDsl, RunQueryDsl};
use rand::Rng;
use diesel::ExpressionMethods;
use reqwest::header;
use std::option::Option;
use serde::Deserialize;
use common::crypto::hash::sha256;
use common::util::auth::Identity;
use common::util::response::ApiResponse;
use crate::class::model::Class;
use crate::user::data_model::course::Root;

use crate::ConnPool;
use crate::schema::{classes, specialities, users};
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
    let mut map_student = HashMap::new();
    map_student.insert("page", 1);
    map_student.insert("pageSize", 5000);
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
                .unwrap()+"/studentData/")
            .query(&map_student)
            .send()
            .await
            .unwrap();
        let res_string = res.text().await.unwrap();
        let mut model: Response = serde_json::from_str(&res_string).unwrap();
        let num = model.data.student_data.len();
        student_data.extend(model.data.student_data.drain(..));
        if num >= 5000{
            let v = map_student.get("page").unwrap();
            map_student.insert("page", v+1);
            continue;
        }
        break;
    }
    //获取课程信息
    let mut map_course = HashMap::new();
    map_course.insert("page", 1);
    map_course.insert("pageSize", 5000);
    map_course.insert("studentDataID", 0);
    map_course.insert("classID", 0);
    let mut course_data = Vec::new();
    loop{
        let res = client
            .get(env::var("MANAGER_BASEURL")
                .unwrap()+"/course/")
            .query(&map_course)
            .send()
            .await
            .unwrap();
        let res_string = res.text().await.unwrap();
        let mut model: Root = serde_json::from_str(&res_string).unwrap();
        let num = model.data.course.len();
        course_data.extend(model.data.course.drain(..));
        if num >= 5000{
            let v = map_course.get("page").unwrap();
            map_course.insert("page", v+1);
            continue;
        }
        break;
    }
    let mut student_class_map = HashMap::new();
    for item in course_data.iter(){
        student_class_map.insert(item.student_data_id.unwrap(),item.class_id.unwrap());
    }
    let mut conn = pool.get().unwrap();
    let user_vec = users::dsl::users.load::<User>(&mut conn).unwrap();
    let insert_data = find_extra_students(&student_data,&user_vec);
    let mut user_vec: Vec<User> = Vec::new();
    for item in insert_data.iter(){
        let s = generate_random_string(8);
        let class_id_option = student_class_map.get(&item.id.unwrap());
        let mut class_idd = 0;
        match class_id_option {
            None => {}
            Some(i) => {
                class_idd = *i;
            }
        }
        let new_user = User{
            id: None,
            username: Option::from(s.clone()),
            password: Option::from(sha256(s.clone())),
            name: Option::from(item.name.clone().unwrap()),
            student_id: Option::from(item.id.clone().unwrap()),
            class_id: Option::from(class_idd),
        };
        user_vec.push(new_user);
    }
    diesel::insert_into(users::dsl::users)
        .values(&user_vec)
        .execute(&mut conn)
        .expect("Error inserting new users");
    HttpResponse::Ok()
        .json(ApiResponse{
            code: 0,
            msg: "success".to_string(),
            data: user_vec,
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
    let login = login_request.into_inner();
    let mut conn = pool.get().unwrap();
    return match users::dsl::users
        .filter(users::username.eq(Option::from(login.username.clone())))
        .select(users::password).first::<Option<String>>(&mut conn) {
        Ok(p) => {
            if p.unwrap()== sha256(login.password) {
                //判断专业并返回对应班级
                let rs = users::dsl::users
                    .inner_join(classes::dsl::classes.on(users::class_id.eq(classes::class_id)))
                    .inner_join(specialities::dsl::specialities.on(classes::speciality_id.eq(specialities::speciality_id)))
                    .select((users::student_id,users::name,specialities::speciality_id,specialities::name))
                    .first::<(Option<i32>,Option<String>, Option<i32>, String)>(&mut conn)
                    .expect("Error loading data");
                let speciality_idd = rs.2.unwrap();
                let cls = classes::dsl::classes
                    .filter(classes::speciality_id.eq(speciality_idd))
                    .load::<Class>(&mut conn)
                    .expect("Error loading data");
                let json_result = serde_json::to_string(&cls).expect("Error serializing to JSON");
                let t = Identity::new(0,0,login.username.clone());
                let mut map_identity = HashMap::new();
                map_identity.insert("user_id",rs.0.unwrap().to_string());
                map_identity.insert("user_name",rs.1.unwrap());
                map_identity.insert("speciality_name",rs.3);
                map_identity.insert("token", t.to_auth_token().unwrap());
                map_identity.insert("classes",json_result);
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
