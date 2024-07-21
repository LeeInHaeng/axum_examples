use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VpnInfo {
    pub country: String,
    pub ip: String,
    pub tcp: String,
    pub udp: String,
    pub sid: String,
    pub hid: String,
}

impl VpnInfo {
    pub fn new() -> VpnInfo {
        VpnInfo {
            country: "Korea Republic of".to_string(),
            ip: String::new(),
            tcp: String::new(),
            udp: String::new(),
            sid: String::new(),
            hid: String::new() 
        }
    }
} 