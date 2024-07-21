use std::fmt::Error;
use scraper::selectable::Selectable;

use super::{lib::request_with_retry, models::VpnInfo};
use url::Url;

pub async fn scrape_info() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    match request_with_retry("https://www.vpngate.net/en/").await {
        Ok(body) => {
            let document = scraper::Html::parse_document(&body);
            let vg_hosts_selector = scraper::Selector::parse("#vg_hosts_table_id").unwrap();

            let mut target_vg_host = None;
            let mut selector_count = 0;
            for vg_host in document.select(&vg_hosts_selector) {
                selector_count += 1;
                if selector_count == 3 {
                    target_vg_host = Some(vg_host);
                    break;
                }
            }

            if target_vg_host.is_none() {
                let err = "target vg host is none";
                eprint!("Failed to parse data: {}", err);
                return Err(Box::new(Error));
            }

            // tbody
            let target_tbody = target_vg_host.unwrap().select(&scraper::Selector::parse("tbody").unwrap()).next();
            if target_tbody.is_none() {
                let err = "target tbody is none";
                eprint!("Failed to parse data: {}", err);
                return Err(Box::new(Error));
            }

            // tr
            for tr in target_tbody.unwrap().select(&scraper::Selector::parse("tr").unwrap()) {
                // td
                let td_selector = scraper::Selector::parse("td").unwrap();
                let mut target_tds = tr.select(&td_selector);
                let first_td = target_tds.next();
                if first_td.is_none() || first_td.unwrap().text().collect::<String>() != "Korea Republic of" {
                    continue;
                }

                // 원래 6인데, 위에서 next 한번 했기 때문에 5
                let config_file_td = target_tds.nth(5);
                if config_file_td.is_none() {
                    let err = "target tr 6 is none";
                    eprint!("Failed to parse data: {}", err);
                    return Err(Box::new(Error));
                }

                let a_selector = scraper::Selector::parse("a").unwrap();
                let config_file_a = config_file_td.unwrap().select(&a_selector).next();
                if config_file_a.is_none() {
                    let err = "config file a is none";
                    eprint!("Failed to parse data: {}", err);
                    return Err(Box::new(Error));
                }

                let config_file_link = config_file_a.unwrap().value().attr("href");
                if config_file_link.is_none() {
                    let err = "config file link is none";
                    eprint!("Failed to parse data: {}", err);
                    return Err(Box::new(Error));
                }

                let parsed_config_file_link = Url::parse(&format!("https://vpngate.net/en/{}", config_file_link.unwrap()).to_string());
                if parsed_config_file_link.is_err() {
                    let err = "config file link parse error";
                    eprint!("Failed to parse data: {}", err);
                    return Err(Box::new(Error));
                }

                let mut vpn_info = VpnInfo::new();
                for (key, value) in parsed_config_file_link.unwrap().query_pairs() {
                    match key.as_ref() {
                        "ip" => {
                            vpn_info.ip = value.to_string();
                        }
                        "tcp" => {
                            vpn_info.tcp = value.to_string();
                        }
                        "udp" => {
                            vpn_info.udp = value.to_string();
                        }
                        "sid" => {
                            vpn_info.sid = value.to_string();
                        }
                        "hid" => {
                            vpn_info.hid = value.to_string();
                        }
                        _ => {}
                    }
                }

                if vpn_info.ip.is_empty() || vpn_info.tcp.is_empty() || vpn_info.udp.is_empty() || vpn_info.sid.is_empty() || vpn_info.hid.is_empty() {
                    continue;
                }

                let result_json = serde_json::to_string(&vpn_info)?;
                result.push(result_json);
            }
        }
        Err(err) => {
            eprint!("Failed to fetch data: {}", err);
            return Err(Box::new(err));
        }
    }

    Ok(result)
}