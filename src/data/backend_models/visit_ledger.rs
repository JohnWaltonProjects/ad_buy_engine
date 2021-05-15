use crate::generate_random_string;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::Utc;
#[cfg(feature = "backend")]
use diesel::{prelude::*, PgConnection, QueryResult};
use uuid::Uuid;
//
// #[cfg_attr(
//     feature = "backend",
//     derive(Queryable, Insertable, AsChangeset, Identifiable),
//     table_name = "visit_ledger",
//     primary_key("id")
// )]
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct VisitLedger {
//     pub id: i64,
// }
//
// #[cfg(feature = "backend")]
// impl VisitLedger {
//     pub fn new(new: Self, conn: &PgConnection) -> QueryResult<usize> {
//         diesel::insert_into(crate::schema::visit_ledger::dsl::visit_ledger)
//             .values(&new)
//             .execute(conn)
//     }
//
//     pub fn delete(model_id: i64, conn: &PgConnection) -> QueryResult<usize> {
//         diesel::delete(crate::schema::visit_ledger::dsl::visit_ledger.find(model_id)).execute(conn)
//     }
//
//     pub fn get(model_id: i64, conn: &PgConnection) -> QueryResult<Self> {
//         crate::schema::visit_ledger::dsl::visit_ledger
//             .find(model_id)
//             .get_result(conn)
//     }
//
//     pub fn delete_all(conn: &PgConnection) -> QueryResult<usize> {
//         diesel::delete(crate::schema::visit_ledger::dsl::visit_ledger).execute(conn)
//     }
//
//     pub fn all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
//         crate::schema::visit_ledger::dsl::visit_ledger.load::<Self>(conn)
//     }
// }
