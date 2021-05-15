#[cfg(feature = "backend")]
use crate::data::backend_models::DatabaseCommunication;
use crate::data::custom_events::CustomConversionEvent;
use crate::data::visit::click_event::ClickEvent;
use crate::data::visit::conversion::Conversion;
use crate::data::visit::geo_ip::GeoIPData;
use crate::data::visit::user_agent::UserAgentData;
use crate::data::visit::Visit;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::{DateTime, NaiveDateTime, Utc};
#[cfg(feature = "backend")]
use diesel::{prelude::*, PgConnection, QueryResult};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "click_identity",
    primary_key("ua_ip_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickIdentityModal {
    pub ua_ip_id: String,
    pub visit_id: i64,
    pub account_id: String,
    pub click_map: String,
}
