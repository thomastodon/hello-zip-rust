use reqwest::{Error, Response};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Computer {
    pub(crate) id: u128,
}

#[derive(Deserialize)]
pub struct GetComputersResponse {
    pub(crate) computers: Vec<Computer>,
}

pub async fn get_computers() -> GetComputersResponse {
    let client = reqwest::Client::new();
    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").ok();
    let computers: Result<Response, Error> = client
        .get("https://ohmnfr.jamfcloud.com/JSSResource/computers")
        .header("accept", "application/json")
        .basic_auth(username, password)
        .send()
        .await;

    let computers_response = match computers {
        Ok(response) => response.json::<GetComputersResponse>().await,
        Err(error) => panic!("Failed request to auth: {:?}", error),
    };

    match computers_response {
        Ok(payload) => payload,
        Err(error) => panic!("Failed to deser response payload: {:?}", error),
    }
}

#[derive(Deserialize)]
pub struct GetComputerByIdResponse {
    pub computer: ComputerDetail,
}

#[derive(Deserialize)]
pub struct ComputerDetail {
    pub general: ComputerGeneral,
    pub hardware: ComputerHardware,
}

#[derive(Deserialize)]
pub struct ComputerGeneral {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ComputerHardware {
    pub model: String,
    pub os_name: String,
}

pub async fn get_computer_by_id(id: &u128) -> GetComputerByIdResponse {
    let client = reqwest::Client::new();
    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").ok();
    let mobile_device_data_subset: Result<Response, Error> = client
        .get(format!("https://ohmnfr.jamfcloud.com/JSSResource/computers/id/{id}"))
        .header("accept", "application/json")
        .basic_auth(username, password)
        .send()
        .await;

    let computer_hardware_response = match mobile_device_data_subset {
        Ok(response) => response.json::<GetComputerByIdResponse>().await,
        Err(error) => panic!("Failed request to auth: {:?}", error),
    };

    match computer_hardware_response {
        Ok(payload) => payload,
        Err(error) => panic!("Failed to deser response payload: {:?}", error),
    }
}