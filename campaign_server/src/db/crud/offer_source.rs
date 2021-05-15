use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::offer_source::OfferSourceModel;
use ad_buy_engine::data::elements::offer_source::OfferSource;
use ad_buy_engine::diesel::insert_into;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::diesel::query_builder::IntoUpdateTarget;
use ad_buy_engine::diesel::result::Error;
use ad_buy_engine::diesel::update;
use ad_buy_engine::uuid::Uuid;

pub fn create_offer_source(
    pool: &PgPool,
    payload: OfferSourceModel,
) -> Result<OfferSourceModel, ApiError> {
    use crate::schema::offer_sources::dsl::offer_sources;

    Ok(insert_into(offer_sources)
        .values(payload)
        .get_result::<OfferSourceModel>(&pool.get()?)?)
}

pub fn update_offer_source(
    pool: &PgPool,
    payload: OfferSourceModel,
) -> Result<OfferSourceModel, ApiError> {
    use crate::schema::offer_sources::dsl::{id as offer_source_id, offer_sources};
    println!("{}", &payload.id.as_str());

    Ok(
        update(offer_sources.filter(offer_source_id.eq(payload.id.clone())))
            .set(payload)
            .get_result::<OfferSourceModel>(&pool.get()?)?,
    )
}
