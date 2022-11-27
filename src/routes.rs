use actix_web::{get, HttpRequest, HttpResponse, post, Responder, Scope, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

pub fn api() -> Scope {
    return web::scope("/api")
        .service(hello)
        .service(jamf_credentials);
}

#[derive(Serialize)]
struct Hello {
    data: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(Hello { data: String::from("hello world!") })
}

#[derive(Deserialize, Serialize)]
struct Credentials {
    username: String,
    password: String,
    url: String,
}

impl Responder for Credentials {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self)
            .unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/jamf/credentials")]
async fn jamf_credentials(credentials: web::Json<Credentials>) -> impl Responder {
    credentials
}