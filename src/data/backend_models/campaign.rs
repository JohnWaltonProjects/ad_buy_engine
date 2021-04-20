use crate::data::elements::campaign::{Campaign, CampaignDestinationType};
use crate::data::elements::funnel::{Funnel, Sequence};
use crate::data::elements::traffic_source::TrafficSource;
use crate::data::lists::click_transition_method::RedirectOption;
use crate::data::lists::CostModel;
use crate::data::work_space::Clearance;
#[cfg(feature = "backend")]
use crate::schema::*;
use crate::Country;
use chrono::{DateTime, NaiveDateTime, Utc};
use either::Either;
use rust_decimal::Decimal;
use url::Url;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "campaigns",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignModel {
    pub id: String,
    pub account_id: String,
    pub clearance: String,
    pub traffic_source: String,
    pub country: String,
    pub name: String,
    pub cost_model: String,
    pub cost_value: String,
    pub redirect_option: String,
    pub campaign_destination: String,
    pub campaign_core: String,
    pub notes: String,
    pub archived: bool,
    pub last_updated: i64,
    pub last_clicked: i64,
    pub hosts: String,
}

impl From<Campaign> for CampaignModel {
    fn from(campaign: Campaign) -> Self {
        to_json_string!(
            clearance; campaign.clearance
            traffic_source; campaign.traffic_source
            country; campaign.country
            cost_model; campaign.cost_model
            cost_value; campaign.cost_value
            redirect_option; campaign.redirect_option
            campaign_destination; campaign.campaign_destination
            campaign_core; campaign.campaign_core
            hosts; campaign.hosts
        );

        let id = campaign.campaign_id.to_string();
        let account_id = campaign.account_id.to_string();

        Self {
            id,
            account_id,
            clearance,
            traffic_source,
            country,
            name: campaign.name,
            cost_model,
            cost_value,
            redirect_option,
            campaign_destination,
            campaign_core,
            notes: campaign.notes,
            archived: campaign.archived,
            last_updated: campaign.last_updated.timestamp(),
            last_clicked: campaign.last_clicked.timestamp(),
            hosts,
        }
    }
}

impl From<CampaignModel> for Campaign {
    fn from(campaign_model: CampaignModel) -> Self {
        from_json_string!(

            clearance; campaign_model.clearance => Clearance
            traffic_source; campaign_model.traffic_source => TrafficSource
            country; campaign_model.country => Country
            cost_model; campaign_model.cost_model => CostModel
            cost_value; campaign_model.cost_value => Decimal
            redirect_option; campaign_model.redirect_option => RedirectOption
            campaign_destination; campaign_model.campaign_destination => CampaignDestinationType
            campaign_core; campaign_model.campaign_core => Either<Funnel, Sequence>
            hosts; campaign_model.hosts => Vec<Url>
        );

        let campaign_id = Uuid::parse_str(&campaign_model.id).expect("SDFg");
        let account_id = Uuid::parse_str(&campaign_model.account_id).expect("SDFg");

        Self {
            campaign_id,
            account_id,
            clearance,
            traffic_source,
            country,
            name: campaign_model.name,
            cost_model,
            cost_value,
            redirect_option,
            campaign_destination,
            campaign_core,
            notes: campaign_model.notes,
            archived: campaign_model.archived,
            last_updated: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(campaign_model.last_updated, 0),
                Utc,
            ),
            last_clicked: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(campaign_model.last_clicked, 0),
                Utc,
            ),
            hosts,
        }
    }
}
