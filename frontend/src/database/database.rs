use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

use super::js_pouchdb::bindings::PouchDB;
use super::types::DatabaseInfo;
use super::utils::log;
use crate::database::errors::FrontendError;
#[cfg(feature = "couch_app")]
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::serde_json::json;
use std::collections::HashMap;
use url::Url;
use wasm_bindgen::JsValue;

pub struct Database {
    js_db: PouchDB,
}

impl Database {
    pub fn new(account_id: &str) -> Database {
        Database {
            js_db: PouchDB::new(account_id.to_string()),
        }
    }

    pub async fn info(&self) -> Result<DatabaseInfo, FrontendError> {
        log("Pouch: getting database info");
        JsFuture::from(self.js_db.info()).await?.try_into()
    }

    pub async fn close(&self) -> Result<(), FrontendError> {
        JsFuture::from(self.js_db.close()).await?;
        Ok(())
    }

    pub async fn replicate(&self, src: Url, target: String) -> Result<(), FrontendError> {
        let options = json!({
            "live": true,
            "retry": true
        });

        JsFuture::from(self.js_db.replicate(
            src.to_string(),
            target,
            JsValue::from_serde(&options)?,
        ))
        .await?;
        Ok(())
    }

    // pub async fn all_docs(&self) -> Result<Vec<Visit>, FrontendError> {
    //     let mut options = HashMap::new();
    //     options.insert("include_docs", true);
    //     options.insert("descending", true);
    //     let options = JsValue::from_serde(&options)?;
    //
    //     let bulk_result = JsFuture::from(self.js_db.allDocs(options)).await?;
    // }
    //
    // pub async fn all_docs_from_range(
    //     &self,
    //     start: i64,
    //     end: i64,
    // ) -> Result<Vec<Visit>, FrontendError> {
    //     let options = format!(
    //         "include_docs: true, descending: true, startkey: '{}', endkey: '{}'",
    //         start, end
    //     );
    //     let options = JsValue::from(&options);
    //
    //     let bulk_result = JsFuture::from(self.js_db.allDocs(options)).await?;
    // }
}
