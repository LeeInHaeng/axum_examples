mod hc;
mod openvpn;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new()
        .merge(hc::get_routes())
        .merge(openvpn::get_routes())
}