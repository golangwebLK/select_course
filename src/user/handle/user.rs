use std::collections::{HashMap, HashSet};
use std::env;
use actix_web::{HttpResponse, Responder, web};
use rand::Rng;
use reqwest::header;
use std::option::Option;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use common::crypto::hash::sha256;
use common::util::auth::Identity;
use common::util::response::ApiResponse;
use crate::user::data_model::course::Root;

use crate::{ConnPoolDiesel, ConnPoolSqlx};
use crate::schema::{users};
use crate::user::data_model::{Response, StudentDatum};
use crate::user::model::User;


pub async fn create_user(
    pool: web::Data<ConnPoolDiesel>
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

#[derive(Debug, Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}
#[derive(Debug, sqlx::FromRow, Serialize)]
struct UserData {
    student_id: Option<i32>,
    name: Option<String>,
    speciality_id: Option<i32>,
    speciality_name: String,
}

#[derive(sqlx::FromRow, Debug,Serialize)]
pub struct ClassSearch {
    pub id: Option<i32>,
    pub class_name: String,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub note: Option<String>,
    pub speciality_id: Option<i32>,
    pub class_id: Option<i32>,
}


pub async fn login(
    login_request: web::Json<LoginRequest>,
    pool: web::Data<ConnPoolSqlx>
) -> impl Responder{
    let conn = pool.get_ref();
    let login = login_request.into_inner();
    let row = sqlx::query(
        "SELECT password FROM users WHERE username = ?",
    )
        .bind(&login.username)
        .fetch_one(conn)
        .await;

    if let Ok(r) = row{
        if r.get::<String,_>(0) != sha256(login.password){
            return HttpResponse::Ok().json(ApiResponse {
                code: 1,
                msg: "login fail".to_string(),
                data: (),
            });
        }
    }

    let rs = sqlx::query_as::<_, UserData>(
        r#"
            SELECT users.student_id, users.name, specialities.speciality_id, specialities.name as speciality_name
            FROM users
            LEFT JOIN classes ON users.class_id = classes.class_id
            LEFT JOIN specialities ON classes.speciality_id = specialities.speciality_id
            WHERE users.username = ?
            "#,
    )
        .bind(&login.username)
        .fetch_one(conn)
        .await
        .expect("Error loading data");

    let result = sqlx::query_as::<_, crate::user_course::model::UserClass>(
        r#"
        SELECT *
        FROM users_classes
        WHERE user_id = ?
        "#,
    )
        .bind(&rs.student_id.unwrap())
        .fetch_all(conn)
        .await;
    let mut map_identity = HashMap::new();
    if let Ok(r) = result{
        if r.len() != 0{
            let user_class = r.get(0).unwrap();
            let cls = sqlx::query_as!(ClassSearch,
            "SELECT * FROM classes WHERE class_id = ?",
            user_class.class_id
            )
                .fetch_one(conn)
                .await
                .expect("Error loading data");
            let json_result = serde_json::to_string(&cls).expect("Error serializing to JSON");
            map_identity.insert("classed", json_result);
            map_identity.insert("status", "true".parse().unwrap());
        }else {
            let cls = sqlx::query_as!(ClassSearch,
            "SELECT * FROM classes WHERE speciality_id = ?",
            rs.speciality_id.unwrap()
            )
                .fetch_all(conn)
                .await
                .expect("Error loading data");
            let json_result = serde_json::to_string(&cls).expect("Error serializing to JSON");
            map_identity.insert("classes", json_result);
            map_identity.insert("status", "false".parse().unwrap());
        }
    }

    let t = Identity::new(0, 0, login.username.clone());
    map_identity.insert("user_id", rs.student_id.unwrap().to_string());
    map_identity.insert("user_name", rs.name.unwrap());
    map_identity.insert("speciality_name", rs.speciality_name);
    map_identity.insert("token", t.to_auth_token().unwrap());
    HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "login success".to_string(),
        data: map_identity,
    })
}
