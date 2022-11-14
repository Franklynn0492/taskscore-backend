use rocket_okapi::JsonSchema;


#[derive(serde::Serialize, JsonSchema)]
pub struct Response<T> where T: Sized {
    pub data: T,
}