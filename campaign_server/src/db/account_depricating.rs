use crate::utils::database::{get_conn, PgPool};
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::invitation::Invitation;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::diesel::update;
use ad_buy_engine::uuid::Uuid;
use std::ops::Deref;

pub fn query_account(pool: &PgPool, _account_id: Uuid) -> Result<AccountModel, ApiError> {
    use crate::schema::accounts::dsl::{accounts, id as account_id};
    Ok(accounts
        .filter(account_id.eq(_account_id.to_string()))
        .first::<AccountModel>(get_conn(pool)?.deref())
        .map_err(|_| ApiError::NotFound("No Account Found".to_string()))?)
}

pub fn return_all_accounts(pool: &PgPool) -> Result<Vec<AccountModel>, ApiError> {
    use crate::schema::accounts::dsl::{accounts, id as account_id};

    match accounts.load::<AccountModel>(&pool.get()?) {
        Ok(res) => Ok(res),
        Err(e) => {
            println!("{:?}", &e);
            Err(ApiError::InternalServerError(format!("{:?}", e)))
        }
    }
}

pub fn update_account_database(
    pool: &PgPool,
    _account_id: Uuid,
    payload: AccountModel,
) -> Result<AccountModel, ApiError> {
    use crate::schema::accounts::dsl::{accounts, id as account_id};
    Ok(
        update(accounts.filter(account_id.eq(_account_id.to_string())))
            .set(payload)
            .get_result::<AccountModel>(&pool.get()?)?,
    )
}
