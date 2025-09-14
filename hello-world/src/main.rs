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

use crate::database::connection;

#[tokio::main]
async fn main() {
    const ADDRESS: &str = "127.0.0.1:8999";
    
    if let Err(e) = utils::logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    } // 初始化日志记录器

    let app: Router<()> = Router::new()
        .merge(routers::api::api_route());

    let listener = match tokio::net::TcpListener::bind(ADDRESS).await {
        Ok(listener) => {
            info!("Successed to bind {}", ADDRESS);
            listener
        }
        Err(_) => {
            error!("failed to bind {}", ADDRESS);
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
