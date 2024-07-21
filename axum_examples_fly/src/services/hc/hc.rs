use axum::response::Html;

pub async fn check_hc() -> Html<&'static str> {
    Html("health check success!")
}