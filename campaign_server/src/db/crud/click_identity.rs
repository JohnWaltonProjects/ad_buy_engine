use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::backend_models::visit::ClickIdentityModal;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
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
    use crate::schema::click_identity::dsl::{click_identity, visit_record_id};

    Ok(
        update(click_identity.filter(visit_record_id.eq(payload.id.clone())))
            .set(payload)
            .get_result::<ClickIdentityModal>(&pool.get()?)?,
    )
}

pub fn create_click_identity(
    pool: &PgPool,
    payload: ClickIdentityModal,
) -> Result<ClickIdentityModal, ApiError> {
    use crate::schema::click_identity::dsl::click_identity;
    Ok(insert_into(click_identity)
        .values(payload)
        .get_result::<ClickIdentityModal>(&pool.get()?)?)
}
