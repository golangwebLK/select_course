use std::collections::HashMap;
use std::env;
use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime};

use diesel::RunQueryDsl;
use reqwest::header;
use common::util::response::ApiResponse;
use crate::ConnPool;
use crate::class::data_model::Root;
use crate::class::model::Class;
use crate::schema::classes::dsl::classes;

pub async fn create_class(
    pool: web::Data<ConnPool>
) -> impl Responder{
    let mut map = HashMap::new();
    map.insert("page",1);
    map.insert("pageSize",50);
    map.insert("specialityID",0);
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
    let mut class_data = Vec::new();
    loop{
        let res = client
            .get(env::var("MANAGER_BASEURL")
                .unwrap()+"/class/")
            .query(&map)
            .send()
            .await
            .unwrap();
        let res_string = res.text().await.unwrap();
        let mut model: Root = serde_json::from_str(&res_string).unwrap();
        let num = model.data.classes.len();
        class_data.extend(model.data.classes.drain(..));
        if num >= 50{
            let v = map.get("page").unwrap();
            map.insert("page",v+1);
            continue;
        }
        break;
    }
    let mut conn = pool.get().unwrap();
    diesel::delete(classes)
        .execute(&mut conn)
        .expect("Error deleting rows from classes table");

    let mut class_vec: Vec<Class> = Vec::new();
    for item in class_data.iter(){
        let start_time = DateTime::parse_from_str(&item.class_time_start, "%Y-%m-%dT%H:%M:%S%:z")
            .unwrap();
        let end_time = DateTime::parse_from_str(&item.class_time_end, "%Y-%m-%dT%H:%M:%S%:z")
            .unwrap();
        let new_class = Class{
            id: None,
            class_name: item.name.clone(),
            start_time: Option::from(start_time.naive_local()),
            end_time: Option::from(end_time.naive_local()),
            note: Option::from(item.note.clone()),
        };
        class_vec.push(new_class);
    }
    diesel::insert_into(classes)
        .values(&class_vec)
        .execute(&mut conn)
        .expect("Error inserting new classes");
    HttpResponse::Ok()
        .json(ApiResponse{
            code: 0,
            msg: "success".to_string(),
            data: class_vec,
        })
}