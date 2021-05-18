#![doc(html_favicon_url = "https://www.pouch.rs/assets/images/favicon.ico")]
#![doc(html_logo_url = "https://www.pouch.rs/assets/images/logo.svg")]

pub mod database;
// pub use super::database::Database;

pub mod types;

pub mod errors;

pub mod js_pouchdb;

pub mod utils;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub mod prelude {
    pub use super::types::DatabaseInfo;
}

pub use self::prelude::*;
