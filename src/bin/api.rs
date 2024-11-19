use axum::{
    extract::Query, http::StatusCode, routing::get, Json, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};

mod program;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/rta", get(rta_handler));

    let addr = SocketAddr::from(([127,0,0,1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, app).await.unwrap();
}

#[derive(Deserialize)]
struct RomanNumber {
    roman_number: String,
}

#[derive(Serialize)]
enum ArabicNumberResponse {
    Ok(u32),
    Error(String),
}


async fn rta_handler(Query(query): Query<RomanNumber>) -> (StatusCode, Json<ArabicNumberResponse>) {
    match program::roman_to_arabic(&query.roman_number.to_uppercase()) {
        Ok(answer) => {
            return (StatusCode::OK ,Json(ArabicNumberResponse::Ok(answer)))
        },
        Err(err) => {
            return (StatusCode::OK ,Json(ArabicNumberResponse::Error(err)))
        }
    }
}

