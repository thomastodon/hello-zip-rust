use actix_web::{get, HttpRequest, HttpResponse, post, Responder, Scope, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};

use crate::jamf_client::{JamfClient};

pub fn api() -> Scope {
    return web::scope("/api")
        .service(hello)
        .service(jamf_credentials)
        .service(jamf_devices);
}

#[get("/hello")]
async fn hello() -> impl Responder {
    web::Json(Hello { data: String::from("hello world!") })
}

#[post("/jamf/credentials")]
async fn jamf_credentials(credentials: web::Json<Credentials>) -> impl Responder {
    credentials
}

#[get("/jamf/devices")]
async fn jamf_devices() -> impl Responder {
    let jamf_client = JamfClient::new();

    let updates = jamf_client.get_mac_os_managed_software_updates()
        .await
        .availableUpdates;
    let latest_available_os = get_latest_semver(updates);

    let computers_response = jamf_client
        .get_computers()
        .await;

    let devices = stream::iter(computers_response.computers)
        .map(|computer| jamf_client.get_computer_by_id(computer.id))
        .buffered(5)
        .map(|response| {
            let computer = response.computer;
            let os_name = computer.hardware.os_name.clone();
            let os_version = computer.hardware.os_version.clone();
            Device {
                device_id: computer.general.id,
                name: computer.general.name.clone(),
                model: computer.hardware.model.clone(),
                os: format!("{os_name} {os_version}"),
                os_is_latest: computer.hardware.os_version.eq(&latest_available_os),
            }
        })
        .collect::<Vec<Device>>()
        .await;

    Devices { devices }
}

fn get_latest_semver(semvers: Vec<String>) -> String {
    semvers.get(0).unwrap().clone()
}

#[derive(Serialize)]
struct Hello {
    data: String,
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


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_get_latest_semver() {
        let semvers = ["13.0.1", "13.0", "12.6.1", "12.6", "12.5.1", "11.7.1", "11.7", "11.6.8"]
            .map(|s| String::from(s))
            .to_vec();

        let latest_semver = get_latest_semver(semvers);
        assert_eq!(latest_semver, "13.0.1");
    }
}