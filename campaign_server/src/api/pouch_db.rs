use crate::serde_json::Value;
use crate::utils::authentication::{decode_jwt, hash, PrivateClaim};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_identity::{Identity, RequestIdentity};
use actix_web::client::Client;
use actix_web::web;
use actix_web::web::{Bytes, Data, Json, Path};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use ad_buy_engine::Url;
use hmac::{Hmac, Mac, NewMac};
use hmacsha1;
use sha1::Sha1;
use std::ops::Deref;

pub async fn replicate(
    req: HttpRequest,
    body: web::Bytes,
    // db: Path<>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let identity = RequestIdentity::get_identity(&req).unwrap();
    let private_claim: PrivateClaim = decode_jwt(&identity).unwrap();

    let database_name = private_claim
        .account_id
        .to_string()
        .chars()
        .filter(|s| *s != '-')
        .collect::<String>();

    let mut new_url = Url::parse("http://host.docker.internal:5984").expect("56h");

    let path = req.uri().path().replace("/visits", "");
    println!("Hi!");
    println!("{}", &path);

    new_url.set_path(&path);
    new_url.set_query(req.uri().query());

    dbg!(&new_url);

    let forwarded_req = Client::new()
        .request_from(new_url.as_str(), req.head())
        .no_decompress();

    let mut res = forwarded_req
        .basic_auth(database_name.clone(), Some(&hash(&database_name)))
        .send_body(body)
        .await
        .map_err(actix_web::error::Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}
//
// pub async fn replicate(
//     req: HttpRequest,
//     body: web::Bytes,
//     // url: web::Data<Url>,
//     // client: web::Data<Client>,
// ) -> Result<HttpResponse, actix_web::error::Error> {
//     let identity = RequestIdentity::get_identity(&req).unwrap();
//     let private_claim: PrivateClaim = decode_jwt(&identity).unwrap();
//
//     let database_name = private_claim
//         .account_id
//         .to_string()
//         .chars()
//         .filter(|s| *s != '-')
//         .collect::<String>();
//
//     let mac = hmacsha1::hmac_sha1(
//         b"df8ef63a71f4622fd029eb9129ef0394",
//         database_name.as_bytes(),
//     );
//
//     let mut new_url = Url::parse("http://host.docker.internal:5984").expect("56h");
//     new_url.set_path(&format!("/{}", database_name));
//     new_url.set_query(req.uri().query());
//
//     // TODO: This forwarded implementation is incomplete as it only handles the inofficial
//     // X-Forwarded-For header but not the official Forwarded one.
//     let forwarded_req = Client::new()
//         .request_from(new_url.as_str(), req.head())
//         .no_decompress();
//
//     // let forwarded_req = if let Some(addr) = req.head().peer_addr {
//     //     forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
//     // } else {
//     //     forwarded_req
//     // };
//
//     let mut res = forwarded_req
//         .header("Accept", "application/json")
//         .header("Content-Type", "application/json; charset=utf-8")
//         .header("X-Auth-CouchDB-Roles", "")
//         .header("X-Auth-CouchDB-UserName", database_name)
//         .header("X-Auth-CouchDB-Token", format!("{:02x?}", mac))
//         .send_body(body)
//         .await
//         .map_err(actix_web::error::Error::from)?;
//
//     let mut client_resp = HttpResponse::build(res.status());
//     // Remove `Connection` as per
//     // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
//     for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
//         client_resp.header(header_name.clone(), header_value.clone());
//     }
//
//     Ok(client_resp.body(res.body().await?))
// }
//
// // pub async fn replicate(
// //     req: HttpRequest,
// //     body: Bytes,
// //     // payload: Json<Value>,
// //     database_name: Path<String>,
// // ) -> Result<HttpResponse, ApiError> {
// //
// //     let identity = RequestIdentity::get_identity(&req);
// //     if let Some(identity) = identity {
// //         let private_claim: PrivateClaim = decode_jwt(&identity).unwrap();
// //         let database_name=private_claim.account_id.to_string().chars().filter(|s| *s != '-').collect::<String>();
// //         let password_hash = hash(&database_name);
// //         let client = actix_web::client::Client::new();
// //         client.get(&format!("http://couch_app:9000/replicate"))
// //         client
// //     } else {
// //     Err(ApiError::Unauthorized("forbidden".to_string()))
// //     }
//
// // // add basic authentication headers
// // // auth on the couchdb server
// //
// // let forwarded_req = client
// //     .request_from(
// //         format!(
// //             "http://couch_app:9000/{}_replicate?",
// //             database_name.into_inner()
// //         )
// //         .as_str(),
// //         req.head(),
// //     )
// //     .no_decompress();
// //
// // let mut res = forwarded_req.send_body(body).await?;
// //
// // let mut client_resp = HttpResponse::build(res.status());
// //
// // for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
// //     client_resp.header(header_name.clone(), header_value.clone());
// // }
//
// // Ok(client_resp.body(res.body().await?))
// // }
