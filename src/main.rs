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

    let port = std::env::var("PORT").unwrap_or("8080".into());
    let bind_addr = format!("0.0.0.0:{port}");

    HttpServer::new(|| App::new().service(get_image).service(get_thumbnail))
        .bind(bind_addr)?
        .run()
        .await
}
