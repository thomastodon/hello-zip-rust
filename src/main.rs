use actix_web::{App, HttpServer};
use dotenv::dotenv;

use hello_zip_rust::routes::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new().service(api())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

