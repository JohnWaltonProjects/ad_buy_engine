use crate::couch_admin::{add_security_document_to_database, create_database, create_user};
use crate::{COUCH_CLIENT, COUCH_SERVER_URI};

#[tokio::test(flavor = "current_thread")]
pub async fn main_test() {
    let res = create_user(COUCH_CLIENT.clone(), "test_user_x", "test_password_x")
        .await
        .unwrap();

    // let res = create_database(res, "test_user_x").await.unwrap();
    // let res = add_security_document_to_database(
    // 	res.0,
    // 	"test_user_x".to_string(),
    // 	"test_user_x".to_string(),
    // )
    // 	.await
    // 	.unwrap();
    // let user_client =
    // 	ad_buy_engine::couch_rs::Client::new(COUCH_SERVER_URI, "test_user_x", "test_password_x")
    // 		.unwrap();
    // let res = user_client.db("test_user_x").await.unwrap();
}
