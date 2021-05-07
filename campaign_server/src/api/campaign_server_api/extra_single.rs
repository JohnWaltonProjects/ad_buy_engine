use crate::api::campaign_server_api::{find_depth, from_request_extract_identity};
use crate::api::campaign_state::find_campaign;
use crate::api::crud::click_identity::write::store_initial_click;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::http::header::{LOCATION, REFERER, USER_AGENT};
use actix_web::web::{block, Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::data::backend_models::visit::VisitModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::funnel::SequenceType;
use ad_buy_engine::data::elements::matrix::MatrixData;
use ad_buy_engine::data::visit::click_event::{ClickEvent, ClickableElement, TerseElement};
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::traversal::Bft;
use ad_buy_engine::Url;
use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub async fn extra_single(
    req: HttpRequest,
    pool: Data<PgPool>,
    app_state: Data<Mutex<HashMap<Uuid, Campaign>>>,
    redis: Data<Addr<RedisActor>>,
    // offer_group_idx: Path<usize>,
    params: Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    let click_identity = from_request_extract_identity(&req, &redis, &pool).await?;
    let conn = pool.get()?;
    let identity = click_identity.clone();
    let mut visit: Visit = block(move || VisitModel::get(identity.visit_id, conn.deref()))
        .await?
        .into();

    if let Some(sequence_type) = click_identity.click_map.seq_type {
        match sequence_type {
            SequenceType::Matrix => {
                let mid = params.get("mid").unwrap();
                let found_node = click_identity.click_map.find_node_in_matrix(mid);
                let selected_node = &found_node.children.first().unwrap().value;
                let mut url = new_string!("");

                match &selected_node.data {
                    MatrixData::Offer(o) => {
                        url = o.url.to_string();
                        visit.push_click_event(ClickEvent::create(ClickableElement::Offer(
                            TerseElement::new(selected_node.id.clone(), None),
                        )));
                    }
                    MatrixData::LandingPage(lp) => {
                        url = lp.url.to_string();
                        visit.push_click_event(ClickEvent::create(ClickableElement::LandingPage(
                            TerseElement::new(selected_node.id.clone(), Some(lp.url.clone())),
                        )));
                    }
                    _ => {}
                }

                let local_pool = pool.clone();
                block(move || {
                    VisitModel::update(
                        visit.id,
                        visit.into(),
                        local_pool.get().expect("T%$F").deref(),
                    )
                })
                .await?;

                Ok(HttpResponse::Found().header(LOCATION, url).finish())
            }

            _ => {
                let offer_click_map = click_identity
                    .click_map
                    .children
                    .first()
                    .expect("G%$tfsdg")
                    .clone();
                if let MatrixData::Offer(o) = offer_click_map.value.data {
                    let redirect_url = o.url.clone();
                    visit.push_click_event(ClickEvent::create(ClickableElement::Offer(
                        TerseElement::new(o.offer_id, Some(o.url)),
                    )));
                    let local_pool = pool.clone();
                    block(move || {
                        VisitModel::update(
                            visit.id,
                            visit.into(),
                            local_pool.get().expect("asdf").deref(),
                        )
                    })
                    .await?;
                    Ok(HttpResponse::Found()
                        .header(LOCATION, redirect_url.as_str())
                        .finish())
                } else {
                    Err(ApiError::InternalServerError("not an offer".to_string()))
                }
            }
        }
    } else {
        Err(ApiError::InternalServerError(
            "no sequence found".to_string(),
        ))
    }
}
