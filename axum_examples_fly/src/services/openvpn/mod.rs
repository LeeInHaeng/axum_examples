mod openvpn;
mod vpngate;
mod lib;
mod models;

use axum::{routing::get, Router};
use openvpn::*;

pub fn get_routes() -> Router {
    Router::new()
        .route("/openvpn", get(get_vpn_info))
}