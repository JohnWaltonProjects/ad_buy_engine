use crate::data::backend_models::click_identity::ClickIdentityModal;
use crate::data::visit::click_map::ClickMap;
#[cfg(feature = "couch")]
use crate::data::visit::Visit;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickIdentity {
    pub ua_ip_id: String,
    pub visit_id: String,
    pub account_id: String,
    pub click_map: ClickMap,
}

impl ClickIdentity {
    pub fn new(account_id: String, visit_id: String, ua: String, ip: IpAddr, cm: ClickMap) -> Self {
        Self {
            ua_ip_id: format!("{}:{}", ua, ip),
            visit_id,
            account_id,
            click_map: cm,
        }
    }
}

use std::str::FromStr;
impl From<ClickIdentityModal> for ClickIdentity {
    fn from(c: ClickIdentityModal) -> Self {
        Self {
            ua_ip_id: c.ua_ip_id,
            visit_id: c.visit_id.to_string(),
            account_id: c.account_id,
            click_map: serde_json::from_str(&c.click_map).expect("TRGF"),
        }
    }
}

impl From<ClickIdentity> for ClickIdentityModal {
    fn from(c: ClickIdentity) -> Self {
        Self {
            ua_ip_id: c.ua_ip_id,
            visit_id: c.visit_id.parse::<i64>().unwrap(),
            account_id: c.account_id,
            click_map: serde_json::to_string(&c.click_map).expect("%^YHGdsfg"),
        }
    }
}
