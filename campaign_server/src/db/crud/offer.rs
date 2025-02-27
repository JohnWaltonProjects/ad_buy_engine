use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::offer::OfferModel;
use ad_buy_engine::data::elements::offer::Offer;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::update;
use uuid::Uuid;

pub fn create_offer(pool: &PgPool, payload: OfferModel) -> Result<OfferModel, ApiError> {
    use crate::schema::offers::dsl::offers;
    Ok(insert_into(offers)
        .values(payload)
        .get_result::<OfferModel>(&pool.get()?)?)
}

pub fn update_offer(pool: &PgPool, payload: OfferModel) -> Result<OfferModel, ApiError> {
    use crate::schema::offers::dsl::{id as offer_id, offers};

    Ok(
        update(offers.filter(offer_id.eq(payload.id.clone())))
            .set(payload)
            .get_result::<OfferModel>(&pool.get()?)?,
    )
}
