use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::backend_models::visit::ClickIdentityModal;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_click_identity(
    pool: &PgPool,
    payload: ClickIdentityModal,
) -> Result<ClickIdentityModal, ApiError> {
    use crate::schema::click_identity::dsl::click_identity;
    Ok(insert_into(click_identity)
        .values(payload)
        .get_result::<ClickIdentityModal>(&pool.get()?)?)
}

pub fn update_click_identity(
    pool: &PgPool,
    payload: ClickIdentityModal,
) -> Result<ClickIdentityModal, ApiError> {
    use crate::schema::click_identity::dsl::{click_identity, ua_ip_id};

    Ok(
        update(click_identity.filter(ua_ip_id.eq(payload.ua_ip_id.clone())))
            .set(payload)
            .get_result::<ClickIdentityModal>(&pool.get()?)?,
    )
}

pub fn get_click_identity(pool: &PgPool, ua_ip_id: String) -> Result<ClickIdentityModal, ApiError> {
    use crate::schema::click_identity::dsl::{click_identity, ua_ip_id};
    Ok(click_identity
        .find(ua_ip_id)
        .get_result::<ClickIdentityModal>(&pool.get()?)?)
}

pub fn load_click_identities_for_cache(pool: &PgPool) -> Result<Vec<ClickIdentity>, ApiError> {
    use crate::schema::click_identity::dsl::{click_identity, visit_id};
    let mut cut_off = Utc::now().timestamp_nanos();
    cut_off - 200_000;

    let response: Vec<ClickIdentityModal> = click_identity
        .filter(visit_id > cut_off)
        .load::<ClickIdentityModal>(&pool.get()?)?;

    Ok(response
        .iter()
        .map(|s| s.into())
        .collect::<Vec<ClickIdentity>>())
}
