use ad_buy_engine::serde_json::{Map, Value};
use serde::Serialize;

#[derive(Default, Serialize, Debug)]
pub struct Selector(Map<String, Value>);
