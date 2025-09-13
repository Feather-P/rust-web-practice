mod routers;
mod utils;
mod handlers;
mod storage;
mod models;
mod database;

use axum::{Router};
use log::{error, info, warn};
use tokio;
use std::{process};

use crate::routers::api::api_route;
use crate::database::connection;
#[tokio::main]
async fn main() {
    if let Err(e) = utils::logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    } // 初始化日志记录器

    let app: Router<()> = Router::new()
        .merge(api_route());

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
    match axum::serve(listener, app).await {
        Ok(_) => {
            info!("Server started successfully");
        }
        Err(e) => {
            error!("Server error: {}", e);
            process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {}
