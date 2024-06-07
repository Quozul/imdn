use actix_web::{App, HttpServer};

use crate::endpoints::get_image::get_image;
use crate::endpoints::get_thumbnail::get_thumbnail;

mod core;
mod endpoints;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    HttpServer::new(|| App::new().service(get_image).service(get_thumbnail))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
