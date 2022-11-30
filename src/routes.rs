use actix_web::{get, HttpRequest, HttpResponse, post, Responder, Scope, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

use crate::jamf_client;

pub fn api() -> Scope {
    return web::scope("/api")
        .service(hello)
        .service(jamf_credentials)
        .service(jamf_devices);
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

#[derive(Serialize)]
struct Devices {
    devices: Vec<Device>,
}

#[derive(Serialize, Clone)]
struct Device {
    device_id: u64,
    name: String,
    model: String,
    os: String,
    os_is_latest: bool,
}

impl Responder for Devices {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self)
            .unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/jamf/devices")]
async fn jamf_devices() -> impl Responder {
    let computers_response = jamf_client::get_computers().await;

    let computer_ids = computers_response
        .computers
        .iter()
        .map(|computer| computer.id)
        .collect::<Vec<u128>>();

    let mut devices: Vec<Device> = vec![];
    for id in &computer_ids {
        let computer = jamf_client::get_computer_by_id(id)
            .await
            .computer;

        let device = Device {
            device_id: computer.general.id,
            name: computer.general.name,
            model: computer.hardware.model,
            os: computer.hardware.os_name,
            os_is_latest: true,
        };
        devices.push(device);
    };

    Devices { devices }
}
