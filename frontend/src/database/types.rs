#[cfg(feature = "couch_app")]
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::serde_json::Value;
use serde::Deserialize;
use std::convert::TryFrom;
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseInfo {
    pub doc_count: i32,
    pub update_seq: i32,
    pub idb_attachment_format: String,
    pub db_name: String,
    pub auto_compaction: bool,
    pub adapter: String,
}

impl DatabaseInfo {
    pub fn new(slim_account_id: String) -> Self {
        DatabaseInfo {
            db_name: slim_account_id,
            adapter: String::from("unknown"),
            idb_attachment_format: String::from("unknown"),
            doc_count: 0,
            update_seq: 0,
            auto_compaction: false,
        }
    }
}

impl TryFrom<JsValue> for DatabaseInfo {
    type Error = super::errors::FrontendError;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let info: DatabaseInfo = value.into_serde().unwrap();
        Ok(info)
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Document {
//     pub _id: String,
//     pub _rev: String,
//     pub data: Visit,
// }

// use pouch;
// use serde_json::json;
// use serde_json::Number;
// use serde_json::Value as SerdeJsonValue;
// impl From<Visit> for Document {
//     fn from(v: Visit) -> Document {
//         Self {
//             _id: v.id.to_string(),
//             _rev: new_string!(""),
//             data: v,
//         }
//     }
// }
//
// impl From<Document> for Visit {
//     fn from(d: Document) -> Self {
//         d.data
//     }
// }
//
// impl TryFrom<Document> for JsValue {
//     type Error = super::errors::FrontendError;
//
//     fn try_from(document: Document) -> Result<Self, Self::Error> {
//         match JsValue::from_serde(&document) {
//             Ok(result) => Ok(result),
//             Err(error) => Err(error.into()),
//         }
//     }
// }
//
// impl TryFrom<JsValue> for Document {
//     type Error = super::errors::FrontendError  fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
//         match js_value.into_serde() {
//             Ok(result) => Ok(result),
//             Err(error) => Err(error.into()),
//         }
//     }
// }

// #[derive(Serialize, Deserialize)]
// pub struct BulkResponse {
//     pub offset: String,
//     pub total_rows: String,
//     pub rows: Vec<ResponseRow>,
// }
// #[derive(Serialize, Deserialize)]
// pub struct ResponseRow {
//     pub doc: Visit,
//     pub id: String,
//     pub key: String,
//     pub value: ResponseValue,
// }
// #[derive(Serialize, Deserialize)]
// pub struct ResponseValue {
//     pub rev: String,
// }
