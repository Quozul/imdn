use crate::app_state::AppState;
use crate::endpoints::get_image::get_image;
use crate::endpoints::get_thumbnail::get_thumbnail;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use tracing::Level;

pub mod app_state;
mod core;
mod endpoints;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let port = std::env::var("PORT").unwrap_or("8080".into());
    let bind_addr = format!("0.0.0.0:{port}");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .wrap(cors)
            .service(get_image)
            .service(get_thumbnail)
    })
    .bind(bind_addr)?
    .run()
    .await
}
