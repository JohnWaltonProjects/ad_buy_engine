#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate ad_buy_engine;

#[macro_use]
pub mod macros;
pub mod dns;
pub use ad_buy_engine::schema;
pub use ad_buy_engine::serde_json;

use crate::server::server;

pub mod api;
pub mod campaign_agent;
pub mod db;
pub mod email_service;
pub mod helper_functions;
pub mod management;
mod private_routes;
mod public_routes;
mod server;
pub mod test_routes;
pub mod tests;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
