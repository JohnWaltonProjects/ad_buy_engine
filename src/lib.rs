#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]

#[macro_use]
pub extern crate lazy_static;
#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate strum_macros;
#[cfg(feature = "backend")]
#[macro_use]
pub extern crate diesel;

#[macro_use]
pub mod macros;
pub mod constant;
pub mod data;
#[cfg(feature = "backend")]
pub mod schema;
pub mod string_manipulation;

pub use chrono;
#[cfg(feature = "backend")]
pub use couch_rs;
pub use derive_more;
pub use dotenv;
pub use env_logger;
pub use envy;
pub use maxminddb;
pub use serde;
pub use serde_json;
pub use time;
#[cfg(feature = "backend")]
pub use tokio;
pub use url::Url;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Country {
    Global,
    ISOCountry(data::lists::country::Country),
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Country::Global => "Global".to_string(),
            Country::ISOCountry(iso_country) => iso_country.to_string(),
        }
    }
}

pub type AError = anyhow::Error;
pub type ISOLanguage = LanguageCode;

pub use crate::data::iso_language::{LanguageCode, ParseError as ISOLangParseError};
pub use either;
pub use ipnet;
pub use rand;
pub use rust_decimal;
pub use strum;
pub use traversal;
pub use uuid;
pub use uuid::Uuid;
pub use weighted_rs;

use boyer_moore_magiclen::BMByte;
use either::Either;
use rand::Rng;
use weighted_rs::{SmoothWeight, Weight};

pub fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InvitationRequest {
    pub email: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub company_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Register {
    pub email: String,
    pub username: String,
    pub team_name: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterAnother {
    pub email: String,
    pub username: String,
    pub password: String,
}
