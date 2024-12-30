use crate::app_state::AppState;
use crate::endpoints::get_image::get_image;
use crate::endpoints::get_thumbnail::get_thumbnail;
use crate::parse_cli::Cli;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use tracing::Level;

pub mod app_state;
mod core;
mod endpoints;
mod parse_cli;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let port = std::env::var("PORT").unwrap_or("8080".into());
    let bind_addr = format!("0.0.0.0:{port}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(
                AppState::new(args.clone()).expect("Failed to initialize application."),
            ))
            .wrap(cors)
            .service(get_image)
            .service(get_thumbnail)
    })
    .bind(bind_addr)?
    .run()
    .await
}
