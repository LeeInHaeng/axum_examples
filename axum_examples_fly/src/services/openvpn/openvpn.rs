use axum::{response::IntoResponse, Json};
use serde_json::Value;

use super::vpngate;

pub async fn get_vpn_info() -> impl IntoResponse {
    let vpngate_get = vpngate::get_scrape_info().await;
    let mut result = Vec::new();

    match vpngate_get {
        Ok(vpn_info_vec) => {
            for vpn_info_json in &vpn_info_vec {
                if let Ok(parsed_json) = serde_json::from_str::<Value>(&vpn_info_json[..]) {
                    result.push(parsed_json);
                } else {
                    eprintln!("Failed to parse JSON data: {}", vpn_info_json);
                }
            }
            Json(result)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Json(result)
        }
    }
}