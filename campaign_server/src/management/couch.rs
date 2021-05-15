use crate::utils::errors::ApiError;
use actix_web::web::Data;
use ad_buy_engine::couch_rs::document::DocumentCollection;
use ad_buy_engine::couch_rs::types::find::FindQuery;
use ad_buy_engine::data::account::Account;
use ad_buy_engine::serde_json::Value;
use std::error::Error;

// pub async fn fetch_from_couch_database(
//     target:i64,
//     account: &Account,
//     couch_client: Data<couch_rs::Client>,
// ) -> Result<(), ApiError> {
// let database=couch_client.db(account.account_id.to_string().as_str());
//     let find=FindQuery::new()
//     Ok(())
// }

pub async fn create_couch_database(
    account: &Account,
    couch_client: Data<ad_buy_engine::couch_rs::Client>,
) -> Result<(), ApiError> {
    let make_db_result = couch_client
        .make_db(account.account_id.to_string().as_str())
        .await?;
    Ok(())
}

pub async fn destroy_couch_database(
    account: &Account,
    couch_client: Data<ad_buy_engine::couch_rs::Client>,
) -> Result<(), ApiError> {
    let make_db_result = couch_client
        .destroy_db(account.account_id.to_string().as_str())
        .await?;
    Ok(())
}
