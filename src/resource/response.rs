use rocket_okapi::JsonSchema;


#[derive(serde::Serialize, JsonSchema)]
struct Response<T> where T: Sized {
    data: T,
}