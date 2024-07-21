use super::vpngate;

pub async fn get_vpn_info() {
    let test = vpngate::get_scrape_info().await;
    println!("{:?}", test);
}