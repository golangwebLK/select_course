use std::collections::HashMap;
use std::env;
use actix_web::{HttpResponse, Responder, web};
use reqwest::header;
use crate::ConnPool;
use crate::user::data_model::Response;

async fn create_class(
    pool: web::Data<ConnPool>
) -> impl Responder{
    let mut map = HashMap::new();
    map.insert("page",1);
    map.insert("pageSize",50);
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
        let mut model: Response = serde_json::from_str(&res_string).unwrap();
        let num = model.data.student_data.len();
        class_data.extend(model.data.student_data.drain(..));
        if num >= 50{
            let v = map.get("page").unwrap();
            map.insert("page",v+1);
            continue;
        }
        break;
    }
    let mut _conn = pool.get().unwrap();
    HttpResponse::Ok().finish()
}