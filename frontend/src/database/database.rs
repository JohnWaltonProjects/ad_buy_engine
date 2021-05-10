use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

use super::errors::Error;
use super::js_pouchdb::bindings::PouchDB;
use super::types::DatabaseInfo;
use super::utils::log;
use crate::database::types::Document;
use wasm_bindgen::JsValue;

pub struct Database {
    js_db: PouchDB,
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database {
            js_db: PouchDB::new(String::from(name)),
        }
    }

    pub async fn info(&self) -> Result<DatabaseInfo, Error> {
        log("Pouch: getting database info");
        JsFuture::from(self.js_db.info()).await?.try_into()
    }

    pub async fn close(&self) -> Result<(), Error> {
        JsFuture::from(self.js_db.close()).await?;
        Ok(())
    }

    pub async fn destroy(&self) -> Result<(), Error> {
        JsFuture::from(self.js_db.destroy()).await?;
        Ok(())
    }

    pub async fn get(&self, id: JsValue) -> Result<Document, Error> {
        JsFuture::from(self.js_db.get(id)).await?.try_into()
    }
}
