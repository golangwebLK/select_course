use std::collections::HashMap;
use std::env;
use std::option::Option;
use actix_web::{HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use reqwest::header;
use common::util::response::ApiResponse;
use crate::ConnPool;
use crate::schema::specialities::dsl::specialities;
use crate::speciality::data_model::Root;
use crate::speciality::model::Speciality;

pub async fn create_speciality(
    pool: web::Data<ConnPool>
) -> impl Responder{
    let mut map = HashMap::new();
    map.insert("page",1);
    map.insert("pageSize",10);
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
    let mut specialities_data = Vec::new();
    loop{
        let res = client
            .get(env::var("MANAGER_BASEURL")
                .unwrap()+"/speciality/")
            .query(&map)
            .send()
            .await
            .unwrap();
        let res_string = res.text().await.unwrap();
        let mut model: Root = serde_json::from_str(&res_string).unwrap();
        let num = model.data.specialities.len();
        specialities_data.extend(model.data.specialities.drain(..));
        if num >= 10{
            let v = map.get("page").unwrap();
            map.insert("page",v+1);
            continue;
        }
        break;
    }
    let mut conn = pool.get().unwrap();
    diesel::delete(specialities)
        .execute(&mut conn)
        .expect("Error deleting rows from classes table");

    let mut specialities_vec: Vec<Speciality> = Vec::new();
    for item in specialities_data.iter(){
        let new_speciality = Speciality{
            id: None,
            name: item.name.clone().unwrap(),
            enable: item.enable,
            note: Option::from(item.note.clone().unwrap()),
            speciality_id: Option::from(item.id),
        };
        specialities_vec.push(new_speciality);
    }
    diesel::insert_into(specialities)
        .values(&specialities_vec)
        .execute(&mut conn)
        .expect("Error inserting new classes");
    HttpResponse::Ok()
        .json(ApiResponse{
            code: 0,
            msg: "success".to_string(),
            data: specialities_vec,
        })
}