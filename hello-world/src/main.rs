mod utils;

use axum::{Router, routing::get};
use log::{error, info, warn};
use tokio;
use std::{process};

#[tokio::main]
async fn main() {
    if let Err(e) = utils::logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    }

    async fn helloworld() -> String {
        "Ciallo~ World".to_string()
    }
    let app: Router<()> = Router::new().route("/", get(helloworld));

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:8999").await {
        Ok(listener) => {
            info!("Successed to bind 127.0.0.1:8999 ");
            listener
        }
        Err(_) => {
            error!("failed to bind 127.0.0.1:8999");
            process::exit(1);
        }
    };
    axum::serve(listener, app).await.unwrap();
}
#[cfg(test)]
mod tests {}
