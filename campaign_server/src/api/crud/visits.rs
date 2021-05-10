use crate::utils::authentication::decode_jwt;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_identity::Identity;
use actix_web::web::{block, Data, Json};
use ad_buy_engine::data::backend_models::visit::VisitModel;
use ad_buy_engine::data::backend_models::visit_ledger::VisitLedger;
use ad_buy_engine::data::visit::Visit;
use std::ops::Deref;
use uuid::Uuid;

pub async fn sync_visits(
    pool: Data<PgPool>,
    identity: Identity,
    payload: Json<Option<i64>>,
) -> Result<Json<Vec<Visit>>, ApiError> {
    let account_id = decode_jwt(&identity.identity().expect("g3qw"))
        .map_err(|e| e)?
        .account_id
        .to_string();

    let local_pool = pool.clone();

    if let Some(timestamp) = payload.into_inner() {
        let mut visits = block(move || {
            VisitModel::all_for_account(account_id, local_pool.get().expect("G%$f").deref())
        })
        .await?
        .into_iter()
        .map(|s| s.into())
        .collect::<Vec<Visit>>();

        visits.retain(|s| s.id > timestamp);

        respond_json(visits)
    } else {
        let visits = block(move || {
            VisitModel::all_for_account(account_id, local_pool.get().expect("G%$f").deref())
        })
        .await?
        .into_iter()
        .map(|s| s.into())
        .collect::<Vec<Visit>>();

        respond_json(visits)
    }
}
