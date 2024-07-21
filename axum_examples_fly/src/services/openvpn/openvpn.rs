use super::vpngate;

pub async fn get_vpn_info() {
    let test = vpngate::scrape_info().await;
    println!("{:?}", test);
}