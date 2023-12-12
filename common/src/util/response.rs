use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>{
    pub code: i16,
    pub msg: String,
    pub data: T
}