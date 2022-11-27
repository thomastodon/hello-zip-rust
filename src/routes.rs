use actix_web::{get, Responder, Scope, web};
use serde::{Serialize};

pub fn api() -> Scope {
    return web::scope("/api")
        .service(hello)
}

#[derive(Serialize)]
struct Hello {
    data: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(Hello { data: String::from("hello world!") })
}