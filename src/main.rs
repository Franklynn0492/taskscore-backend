
use rocket::serde::json::Json;

//mod models;

#[macro_use] extern crate rocket;
extern crate dotenv;

#[get("/")]
fn hello() -> Json<String> {
    Json("Hello, world!".to_owned())
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("Route not found")
}

#[rocket::main]
async fn main() {
    let context_root = "/TaskScore/rest";

    let _ = rocket::build()

    //.manage(connection::init_pool())
    .mount(context_root, routes![hello])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}