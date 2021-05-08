use crate::generate_random_string;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::Utc;
#[cfg(feature = "backend")]
use diesel::{prelude::*, PgConnection, QueryResult};
use uuid::Uuid;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "linked_conversion",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkedConversion {
    /// Subid that is posted from Offer Source
    pub id: String,
    pub visit_id: i64,
    pub campaign_id: String,
    pub offer_id: String,
    pub created_at: i64,
}

// todo if multiple clicks do not want to make multiple linked_conversion records in database, need to
#[cfg(feature = "backend")]
impl LinkedConversion {
    pub fn create(visit_id: i64, campaign_id: &Uuid, offer_id: &Uuid) -> Self {
        Self {
            id: generate_random_string(24),
            visit_id: visit_id,
            campaign_id: campaign_id.to_string(),
            offer_id: offer_id.to_string(),
            created_at: Utc::now().timestamp(),
        }
    }

    pub fn new(new: Self, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(crate::schema::linked_conversion::dsl::linked_conversion)
            .values(&new)
            .execute(conn)
    }

    pub fn delete(model_id: String, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(crate::schema::linked_conversion::dsl::linked_conversion.find(model_id))
            .execute(conn)
    }

    // pub fn update(model_id: String, new: Self, conn: &PgConnection) -> QueryResult<usize> {
    //     diesel::update(crate::schema::linked_conversion::dsl::linked_conversion.find(model_id))
    //         .set(new)
    //         .execute(conn)
    // }

    pub fn get(model_id: String, conn: &PgConnection) -> QueryResult<Self> {
        crate::schema::linked_conversion::dsl::linked_conversion
            .find(model_id)
            .get_result(conn)
    }
    //
    // pub fn update_and_get(model_id: String, new: Self, conn: &PgConnection) -> QueryResult<Self> {
    //     diesel::update(crate::schema::linked_conversion::dsl::visits.find(model_id))
    //         .set(&new)
    //         .get_result(conn)
    // }

    pub fn delete_all(conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(crate::schema::linked_conversion::dsl::linked_conversion).execute(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        crate::schema::linked_conversion::dsl::linked_conversion.load::<Self>(conn)
    }
}
