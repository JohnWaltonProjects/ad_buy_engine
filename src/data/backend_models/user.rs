use crate::data::user::User;
#[cfg(feature = "backend")]
use crate::schema::*;
use crate::UserResponse;
use chrono::{Local, NaiveDateTime, Utc};
use uuid::Uuid;
#[cfg(feature = "backend")]
use diesel::{PgConnection, QueryResult, RunQueryDsl};

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "users",
    primary_key("id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModel {
    pub id: String,
    pub account_id: String,
    pub email: String,
    pub password: String,
    pub last_updated: i64,
}

#[cfg(feature = "backend")]
impl UserModel {
    pub fn delete_all(conn:&PgConnection)->QueryResult<usize> {
        diesel::delete(users::dsl::users).execute(conn)
    }
}

impl From<User> for UserModel {
    fn from(u: User) -> Self {
        Self {
            id: u.user_id.to_string(),
            account_id: u.account_id.to_string(),
            email: u.email,
            password: u.password,
            last_updated: Utc::now().timestamp(),
        }
    }
}

impl From<UserModel> for User {
    fn from(u: UserModel) -> Self {
        Self {
            user_id: Uuid::parse_str(&u.id).unwrap(),
            account_id: Uuid::parse_str(&u.account_id).unwrap(),
            email: u.email,
            password: u.password,
        }
    }
}

impl From<UserModel> for UserResponse {
    fn from(m: UserModel) -> Self {
        Self {
            id: Uuid::parse_str(&m.id).expect("G%$sef"),
            account_id: Uuid::parse_str(&m.account_id).expect("R3gsaef"),
            email: m.email,
        }
    }
}
