mod openvpn;
mod vpngate;
mod lib;
mod models;

use axum::{routing::get, Router};
use openvpn::*;
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn get_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/openvpn", get(get_vpn_info))
        .layer(cors)
}