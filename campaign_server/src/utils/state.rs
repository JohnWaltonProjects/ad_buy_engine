use actix_web::web::Data;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

pub type State<'a, T> = HashMap<&'a str, T>;
pub type AppState = Data<Mutex<HashMap<Uuid, Campaign>>>;

pub fn init_state<'a>(campaigns: Vec<Campaign>) -> Data<Mutex<HashMap<Uuid, Campaign>>> {
    let mut hm = HashMap::new();
    for camp in campaigns {
        let id = camp.campaign_id;
        hm.insert(id, camp);
    }
    Data::new(Mutex::new(hm))
}
