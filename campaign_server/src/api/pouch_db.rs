use crate::utils::authentication::{decode_jwt, PrivateClaim};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_identity::Identity;
use actix_web::web::{Bytes, Data, Json, Path};
use actix_web::{HttpRequest, HttpResponse};

pub async fn replicate(
    req: HttpRequest,
    body: Bytes,
    id: Identity,
    database_name: Path<String>,
    // couch_client:Data<couch_rs::Client>,
    client: Data<awc::Client>,
) -> Result<HttpResponse, ApiError> {
    let restored_identity: PrivateClaim =
        decode_jwt(&id.identity().expect("g3qw")).map_err(|e| e)?;
    assert_eq!(
        database_name.0.as_str(),
        restored_identity.account_id.to_string().as_str()
    );

    let forwarded_req = client
        .request_from(
            format!("couch_database:5984/{}", database_name.into_inner()).as_str(),
            req.head(),
        )
        .no_decompress();

    let mut res = forwarded_req.send_body(body).await?;

    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}
