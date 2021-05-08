use crate::api::campaign_state::find_campaign;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_ok;
use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::http::header::{LOCATION, REFERER, USER_AGENT};
use actix_web::web::{block, Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::data::backend_models::linked_conversion::LinkedConversion;
use ad_buy_engine::data::backend_models::visit::VisitModel;
use ad_buy_engine::data::backend_models::visit_ledger::VisitLedger;
use ad_buy_engine::data::backend_models::DatabaseCommunication;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::matrix::MatrixData;
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::conversion::Conversion;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::Url;
use chrono::Local;
use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub async fn action(
    req: HttpRequest,
    pool: Data<PgPool>,
    params: Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let sid = params.get("sid").cloned();

    if let Some(subid) = sid {
        let local_pool = pool.clone();
        let restored_linked_conversion =
            block(move || LinkedConversion::get(subid, local_pool.get().expect("T$G%H^%").deref()))
                .await?;

        let visit_id = restored_linked_conversion.visit_id;
        let local_pool = pool.clone();
        let mut restored_visit: Visit =
            block(move || VisitModel::get(visit_id, local_pool.get().expect("G%$RT").deref()))
                .await?
                .into();
        let conversion = Conversion {
            postback_url_parameters: params.into_inner(),
            offer_id: Uuid::parse_str(&restored_linked_conversion.offer_id.clone()).expect("^H%Y"),
            postback_timestamp: Local::now().naive_local(),
        };
        restored_visit.conversions.push(conversion);
        let restored_visit_id = restored_visit.id;

        let local_pool = pool.clone();
        let block_result = block(move || {
            VisitModel::update(
                restored_visit.id,
                restored_visit.into(),
                local_pool.get().expect("t46ghrFD").deref(),
            )
        })
        .await?;

        let local_pool = pool.clone();
        let block_result = block(move || {
            VisitLedger::new(
                VisitLedger {
                    id: restored_visit_id,
                },
                local_pool.get().expect("G%$TW").deref(),
            )
        })
        .await?;
    }

    respond_ok()
}
