use reqwest::{Error, Response};
use serde::{Deserialize};


const JAMF_BASE_URL: &str = "https://ohmnfr.jamfcloud.com";

pub(crate) struct JamfClient {
    username: String,
    password: Option<String>,
    client: reqwest::Client,
}

impl JamfClient {
    pub(crate) fn new() -> JamfClient {
        let username = match std::env::var("USERNAME") {
            Ok(val) => val,
            Err(error) => panic!("Failed to read username from .env file: {:?}", error),
        };

        let password = match std::env::var("PASSWORD") {
            Ok(val) => Some(val),
            Err(error) => panic!("Failed to read password from .env file: {:?}", error),
        };

        let client = reqwest::Client::new();

        JamfClient { username, password, client }
    }

    async fn get_auth_token(&self) -> GetAuthTokenResponse {
        let get_auth_token: Result<Response, Error> = self.client
            .post(format!("{JAMF_BASE_URL}/api/v1/auth/token"))
            .header("accept", "application/json")
            .basic_auth(self.username.clone(), self.password.clone())
            .send()
            .await;

        let get_auth_token_response = match get_auth_token {
            Ok(response) => response.json::<GetAuthTokenResponse>().await,
            Err(error) => panic!("Failed request: {:?}", error),
        };

        match get_auth_token_response {
            Ok(payload) => payload,
            Err(error) => panic!("Failed to deser response payload: {:?}", error),
        }
    }

    pub(crate) async fn get_computers(&self) -> GetComputersResponse {
        let get_computers: Result<Response, Error> = self.client
            .get(format!("{JAMF_BASE_URL}/JSSResource/computers"))
            .header("accept", "application/json")
            .basic_auth(self.username.clone(), self.password.clone())
            .send()
            .await;

        let computers_response = match get_computers {
            Ok(response) => response.json::<GetComputersResponse>().await,
            Err(error) => panic!("Failed request: {:?}", error),
        };

        match computers_response {
            Ok(payload) => payload,
            Err(error) => panic!("Failed to deser response payload: {:?}", error),
        }
    }

    pub(crate) async fn get_computer_by_id(&self, id: u64) -> GetComputerByIdResponse {
        let get_computer_by_id: Result<Response, Error> = self.client
            .get(format!("{JAMF_BASE_URL}/JSSResource/computers/id/{id}"))
            .header("accept", "application/json")
            .basic_auth(self.username.clone(), self.password.clone())
            .send()
            .await;

        let computer_hardware_response = match get_computer_by_id {
            Ok(response) => response.json::<GetComputerByIdResponse>().await,
            Err(error) => panic!("Failed request: {:?}", error),
        };

        match computer_hardware_response {
            Ok(payload) => payload,
            Err(error) => panic!("Failed to deser response payload: {:?}", error),
        }
    }

    pub(crate) async fn get_mac_os_managed_software_updates(&self) -> GetMacOsManagedSoftwareUpdatesResponse {
        let token = self.get_auth_token().await.token;

        let get_mac_os_managed_software_updates: Result<Response, Error> = self.client
            .get(format!("{JAMF_BASE_URL}/api/v1/macos-managed-software-updates/available-updates"))
            .header("accept", "application/json")
            .bearer_auth(token)
            .send()
            .await;

        let get_mac_os_managed_software_updates_response = match get_mac_os_managed_software_updates {
            Ok(response) => response.json::<GetMacOsManagedSoftwareUpdatesResponse>().await,
            Err(error) => panic!("Failed request: {:?}", error),
        };

        match get_mac_os_managed_software_updates_response {
            Ok(payload) => payload,
            Err(error) => panic!("Failed to deser response payload: {:?}", error),
        }
    }
}

#[derive(Deserialize)]
struct GetAuthTokenResponse {
    token: String,
}

#[derive(Deserialize)]
pub(crate) struct Computer {
    pub(crate) id: u64,
}

#[derive(Deserialize)]
pub(crate) struct GetComputersResponse {
    pub(crate) computers: Vec<Computer>,
}

#[derive(Deserialize)]
pub(crate) struct GetComputerByIdResponse {
    pub computer: ComputerDetail,
}

#[derive(Deserialize)]
pub(crate) struct ComputerDetail {
    pub general: ComputerGeneral,
    pub hardware: ComputerHardware,
}

#[derive(Deserialize)]
pub(crate) struct ComputerGeneral {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub(crate) struct ComputerHardware {
    pub model: String,
    pub os_name: String,
    pub os_version: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub(crate) struct GetMacOsManagedSoftwareUpdatesResponse {
    pub(crate) availableUpdates: Vec<String>,
}
