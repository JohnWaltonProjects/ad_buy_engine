// use std::convert::TryInto;
// use wasm_bindgen_futures::JsFuture;
//
// use super::js_pouchdb::bindings::PouchDB;
// use super::types::DatabaseInfo;
// use super::utils::log;
// use crate::database::errors::FrontendError;
// use crate::database::js_pouchdb::bindings::replicate;
// #[cfg(feature = "couch_app")]
// use ad_buy_engine::data::visit::Visit;
// use ad_buy_engine::serde_json::json;
// use std::collections::HashMap;
// use url::Url;
// use wasm_bindgen::JsValue;
//
// pub fn version() -> &'static str {
//     env!("CARGO_PKG_VERSION")
// }
//
// pub struct Database {
//     js_db: PouchDB,
// }
//
// impl Database {
//     pub fn new(account_id: &str) -> Database {
//         Database {
//             js_db: PouchDB::new(account_id.to_string()),
//         }
//     }
//
//     pub async fn info(&self) -> Result<DatabaseInfo, FrontendError> {
//         log("DB: getting database info");
//         JsFuture::from(self.js_db.info()).await?.try_into()
//     }
//
//     pub async fn replicate(&self, slim_account_id: String) -> Result<(), FrontendError> {
//         let mut options = HashMap::new();
//         options.insert("live", true);
//         options.insert("retry", true);
//
//         log("DB: Sync");
//         JsFuture::from(replicate(slim_account_id)).await?;
//         Ok(())
//     }
// }
