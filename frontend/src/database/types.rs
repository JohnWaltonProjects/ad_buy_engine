use serde::Deserialize;
use serde_json::Value;
use std::convert::TryFrom;
use wasm_bindgen::JsValue;

use super::errors::Error;
use ad_buy_engine::data::visit::Visit;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseInfo {
    pub doc_count: i32,
    pub update_seq: i32,
    pub idb_attachment_format: String,
    pub db_name: String,
    pub auto_compaction: bool,
    pub adapter: String,
}

impl Default for DatabaseInfo {
    fn default() -> Self {
        DatabaseInfo {
            db_name: String::from("unknown"),
            adapter: String::from("unknown"),
            idb_attachment_format: String::from("unknown"),
            doc_count: 0,
            update_seq: 0,
            auto_compaction: false,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub _id: String,
    pub _rev: String,
    pub data: Visit,
}

impl From<Visit> for Document {
    fn from(v: Visit) -> Document {
        Self {
            _id: v.id.to_string(),
            _rev: new_string!(""),
            data: v,
        }
    }
}

impl From<Document> for Visit {
    fn from(d: Document) -> Self {
        d.data
    }
}

impl TryFrom<Document> for JsValue {
    type Error = super::errors::Error;

    fn try_from(document: Document) -> Result<Self, Self::Error> {
        match JsValue::from_serde(&document) {
            Ok(result) => Ok(result),
            Err(error) => Err(error.into()),
        }
    }
}

impl TryFrom<JsValue> for Document {
    type Error = super::errors::Error;

    fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
        match js_value.into_serde() {
            Ok(result) => Ok(result),
            Err(error) => Err(error.into()),
        }
    }
}
