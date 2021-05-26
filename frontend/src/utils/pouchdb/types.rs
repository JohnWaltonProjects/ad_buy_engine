use ad_buy_engine::serde_json::Value;
use serde::Deserialize;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

use super::errors::Error;

#[derive(Deserialize, Debug)]
pub struct DatabaseInfo {
    pub doc_count: i32,
    pub update_seq: i32,
    pub idb_attachment_format: String,
    pub db_name: String,
    pub auto_compaction: bool,
    pub adapter: String,
}

impl DatabaseInfo {
    pub fn new(name: String) -> DatabaseInfo {
        Self {
            doc_count: 0,
            update_seq: 0,
            idb_attachment_format: "".to_string(),
            db_name: name,
            auto_compaction: false,
            adapter: "".to_string(),
        }
    }
}

impl TryFrom<JsValue> for DatabaseInfo {
    type Error = super::errors::Error;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let info: DatabaseInfo = value.into_serde().unwrap();
        Ok(info)
    }
}
//
// #[derive(Deserialize, Debug)]
// pub struct Document<T> {
// 	pub _id: String,
// 	pub _rev: String,
// 	pub data: T,
// }
//
// impl<T> TryFrom<JsValue> for Document<T> {
// 	type Error = super::errors::Error;
// 	fn try_from(value: JsValue) -> Result<Self, Self::Error> {
// 		let _raw_doc: Value = value.into_serde().unwrap();
// 		// TODO convert data into document type
// 		// let data: T = serde_json::from_value(value).unwrap();
// 		Err(Error::Pouch("Not implemented yet"))
// 	}
// }
