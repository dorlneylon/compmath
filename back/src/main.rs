#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::core::mat::Mat;
use crate::core::models;
use tower_http::cors::{CorsLayer};
use axum::{Router, Json};
use axum::routing::post;

pub mod core;
pub mod tests;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let app = Router::new()
        .route("/", post(handler))
        .layer(CorsLayer::permissive());

    axum::serve(listener, app).await.unwrap();
}

async fn handler(Json(buf): Json<models::Request>) -> Json<models::Response> {
    let request = buf;
    let mut A = Mat::from(request.A);
    let response = core::eq_solver::gauss_seidel(&mut A, &request.b, request.eps);

    if let Err(msg) = response {
        println!("{}", msg);
        return Json(models::Response { x: vec![], acc: vec![], eps: 0.0, iters: 0, error: msg });
    }

    let response = response.unwrap();

    Json(response)
}