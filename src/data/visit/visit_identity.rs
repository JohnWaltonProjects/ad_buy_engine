use crate::data::backend_models::visit::ClickIdentityModal;
use crate::data::visit::click_map::ClickMap;
use crate::data::visit::Visit;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickIdentity {
    pub ua_ip_id: String,
    pub visit_id: i64,
    pub click_map: ClickMap,
}

impl ClickIdentity {
    pub fn new(visit_id: i64, ua: String, ip: IpAddr, cm: ClickMap) -> Self {
        Self {
            ua_ip_id: format!("{}:{}", ua, ip),
            visit_id,
            click_map: cm,
        }
    }
}

use std::str::FromStr;
impl From<ClickIdentityModal> for ClickIdentity {
    fn from(c: ClickIdentityModal) -> Self {
        Self {
            ua_ip_id: c.ua_ip_id,
            visit_id: c.visit_id,
            click_map: serde_json::from_str(&c.click_map).expect("TRGF"),
        }
    }
}

impl From<ClickIdentity> for ClickIdentityModal {
    fn from(c: ClickIdentity) -> Self {
        Self {
            ua_ip_id: c.ua_ip_id,
            visit_id: c.visit_id,
            click_map: serde_json::to_string(&c.click_map).expect("%^YHGdsfg"),
        }
    }
}

// impl VisitIdentity {
//     pub fn new(visit: Visit, cm: ClickMap) -> Self {
//         Self {
//             visit_record_id: visit.meta.click_id,
//             date: chrono::Local::now().timestamp(),
//             ua: visit.user_agent_data.user_agent_string.clone(),
//             ip: visit.geo_ip_data.ip,
//             click_map: cm,
//         }
//     }
//
//     pub fn get_next_url(&self, referring_url: &str) {}
//
//     pub fn get_initial_url(&self) -> String {
//         match &self.click_map {
//             ClickMap::DL(a) => a.offer_url.to_string(),
//             ClickMap::LP(b) => b.landing_page_url.to_string(),
//             ClickMap::MV(c) => c.root_url.to_string(),
//         }
//     }
// }
