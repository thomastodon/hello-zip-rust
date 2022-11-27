#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use hello_zip_rust::routes::api;
    use serde_json::Value;

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
}