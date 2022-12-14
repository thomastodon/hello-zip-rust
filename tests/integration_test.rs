#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use dotenv::dotenv;
    use hello_zip_rust::routes::api;
    use serde_json::Value;
    use serde::{Serialize};

    #[actix_web::test]
    async fn test_get_hello() {
        let app = test::init_service(App::new().service(api())).await;

        let req = test::TestRequest::get().uri("/api/hello").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let actual_response_payload: Value = test::read_body_json(resp).await;
        let expected_response_payload: Value = serde_json::from_str(r#"{ "data": "hello world!" }"#)
            .unwrap();
        assert_eq!(actual_response_payload, expected_response_payload);
    }

    #[actix_web::test]
    async fn test_post_jamf_credentials() {
        let app = test::init_service(App::new().service(api())).await;

        #[derive(Serialize)]
        struct JamfCredentialsRequestPayload {
            username: String,
            password: String,
            url: String,
        }

        let payload = JamfCredentialsRequestPayload {
            username: String::from("tshouler"),
            password: String::from("this_is_a_secret"),
            url: String::from("base_url"),
        };

        let req = test::TestRequest::post()
            .set_json(payload)
            .uri("/api/jamf/credentials")
            .to_request();

        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());

        let expected_response_body = r#"{
            "username": "tshouler",
            "password": "this_is_a_secret",
            "url": "base_url"
        }"#;
        let actual_response_payload: Value = test::read_body_json(response).await;
        let expected_response_payload: Value = serde_json::from_str(expected_response_body).unwrap();
        assert_eq!(actual_response_payload, expected_response_payload);
    }

    #[actix_web::test]
    async fn test_get_jamf_devices() {
        dotenv().ok();

        let app = test::init_service(App::new().service(api())).await;

        let req = test::TestRequest::get().uri("/api/jamf/devices").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let actual_response_payload: Value = test::read_body_json(resp).await;
        let expected_response_body = r#"{"devices": [
        { "device_id": 1, "name": "Mac mini", "model": "Mac mini (2018)", "os": "macOS 12.6.0", "os_is_latest": false },
        { "device_id": 2, "name": "Joshua???s MacBook Pro (2)", "model": "MacBook Pro (13-inch, M1, 2020)", "os": "macOS 12.6.0", "os_is_latest": false },
        { "device_id": 3, "name": "Gabbi???s MacBook Pro", "model": "MacBook Pro (15-inch, 2018)", "os": "macOS 12.6.0", "os_is_latest": false },
        { "device_id": 4, "name": "Ashley???s MacBook Pro", "model": "MacBook Pro (14-inch, 2021)", "os": "macOS 12.5.0", "os_is_latest": false },
        { "device_id": 5, "name": "Nicholas???s MacBook Pro", "model": "MacBook Pro (16-inch, 2021)", "os": "macOS 12.5.0", "os_is_latest": false }
        ]}"#;
        let expected_response_payload: Value = serde_json::from_str(expected_response_body).unwrap();
        assert_eq!(actual_response_payload, expected_response_payload);
    }
}