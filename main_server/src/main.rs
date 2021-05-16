#[macro_use]
extern crate ad_buy_engine;

pub use ad_buy_engine::schema;

mod server;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // server().await
    Ok(())
}
