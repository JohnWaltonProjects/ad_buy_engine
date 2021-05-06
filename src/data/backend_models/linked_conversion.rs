#[cfg(feature = "backend")]
use crate::schema::*;
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "linked_conversion",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkedConversion {
    pub id: String,
    pub campaign_id: String,
    pub offer_id: String,
    pub created_at: i64,
}
// needs to link offer
// need to find campaign with that offer
