use std::error::Error;
use std::time::Duration;

use poem::{EndpointExt, get, listener::TcpListener, Route, Server};
use poem::middleware::Cors;

use crate::endpoints::get_image::get_image;
use crate::endpoints::get_thumbnail::get_thumbnail;

mod core;
mod endpoints;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug,imdn=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/api/image/:path", get(get_image))
        .at("/api/thumbnail/:path", get(get_thumbnail))
        .with(Cors::new());

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await?;

    Ok(())
}
