pub mod account;
pub mod campaign;
pub mod click_identity;
pub mod funnel;
pub mod invitation;
pub mod landing_page;
pub mod linked_conversion;
pub mod live_campaign_table;
pub mod offer;
pub mod offer_source;
pub mod traffic_source;
pub mod user;
pub mod visit_ledger;
use super::backend_models::{
    account::AccountModel, campaign::CampaignModel, funnel::FunnelModel, invitation::Invitation,
    landing_page::LandingPageModel, offer::OfferModel, offer_source::OfferSourceModel,
    traffic_source::TrafficSourceModel, user::UserModel,
};

use crate::data::backend_models::click_identity::ClickIdentityModal;
use crate::data::backend_models::linked_conversion::LinkedConversion;
use crate::data::visit::visit_identity::ClickIdentity;

#[cfg(feature = "backend")]
use crate::schema::emails;
#[cfg(feature = "backend")]
use diesel::{prelude::*, PgConnection, QueryResult, RunQueryDsl};
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "emails",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailModel {
    pub id: String,
}

#[cfg(feature = "backend")]
impl EmailModel {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        emails::dsl::emails.load(conn)
    }

    pub fn delete_all(conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(emails::dsl::emails).execute(conn)
    }
}

pub trait Accountable {}

#[cfg(feature = "backend")]
pub trait AccountableDBComm<T> {
    fn all_by_last_updated(acc_id: String, conn: &PgConnection) -> QueryResult<Vec<T>>;
    fn all_for_account(acc_id: String, conn: &PgConnection) -> QueryResult<Vec<T>>;
    fn delete_all_for_account(acc_id: String, conn: &PgConnection) -> QueryResult<usize>;
}

#[cfg(feature = "backend")]
pub trait DatabaseCommunication<T> {
    fn new(new: T, conn: &PgConnection) -> QueryResult<usize>;
    fn delete(model_id: String, conn: &PgConnection) -> QueryResult<usize>;
    fn update(model_id: String, new: T, conn: &PgConnection) -> QueryResult<usize>;
    fn get(model_id: String, conn: &PgConnection) -> QueryResult<T>;
    fn update_and_get(model_id: String, new: T, conn: &PgConnection) -> QueryResult<T>;
    fn delete_all(conn: &PgConnection) -> QueryResult<usize>;
    fn all(conn: &PgConnection) -> QueryResult<Vec<T>>;
}

#[cfg(feature = "backend")]
impl_database_communication!(
    LinkedConversion, linked_conversion
    ClickIdentityModal, click_identity
    AccountModel, accounts
    UserModel, users
    CampaignModel, campaigns
    FunnelModel, funnels
    TrafficSourceModel, traffic_sources
    LandingPageModel, landing_pages
    OfferModel, offers
    OfferSourceModel, offer_sources
);

#[cfg(feature = "backend")]
impl_accountable_database_communication!(
    UserModel, users
    CampaignModel, campaigns
    FunnelModel, funnels
    TrafficSourceModel, traffic_sources
    LandingPageModel, landing_pages
    OfferModel, offers
    OfferSourceModel, offer_sources
);
