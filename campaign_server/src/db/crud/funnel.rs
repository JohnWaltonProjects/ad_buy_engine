use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::account::AccountModel;
use ad_buy_engine::data::backend_models::funnel::FunnelModel;
use ad_buy_engine::data::elements::funnel::Funnel;
use ad_buy_engine::diesel::insert_into;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::diesel::query_builder::IntoUpdateTarget;
use ad_buy_engine::diesel::update;
use ad_buy_engine::uuid::Uuid;

pub fn create_funnel(pool: &PgPool, payload: FunnelModel) -> Result<FunnelModel, ApiError> {
    use crate::schema::funnels::dsl::funnels;
    Ok(insert_into(funnels)
        .values(payload)
        .get_result::<FunnelModel>(&pool.get()?)?)
}

pub fn update_funnel(pool: &PgPool, payload: FunnelModel) -> Result<FunnelModel, ApiError> {
    use crate::schema::funnels::dsl::{funnels, id as funnel_id};

    Ok(update(funnels.filter(funnel_id.eq(payload.id.clone())))
        .set(payload)
        .get_result::<FunnelModel>(&pool.get()?)?)
}
