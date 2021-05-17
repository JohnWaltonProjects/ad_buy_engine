use crate::api::campaign_server_api::action::action;
use crate::api::campaign_server_api::click::process_initial_click;
use crate::api::campaign_server_api::extra_multiple::extra_multiple;
use crate::api::campaign_server_api::extra_single::extra_single;
use crate::api::crud::click_identity::write::create_click_identity;
use crate::db::crud::click_identity::load_click_identities_for_cache;
use crate::helper_functions::{rate_limit, ssl_config};
use crate::management::couch;
use crate::management::couch::create_couch_database;
use crate::private_routes::private_routes;
use crate::public_routes::public_routes;
use crate::utils::authentication::get_identity_service;
use crate::utils::cache::add_cache;
use crate::utils::database::{establish_connection, PgPool};
use crate::utils::state::init_state;
use actix::Addr;
use actix_cors::Cors;
use actix_files::Files;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_redis::RedisActor;
use actix_service::Service;
use actix_web::client::Client;
use actix_web::http::header;
use actix_web::http::header::HOST;
use actix_web::web::{get, post, resource, route, scope, service, Data, JsonConfig, Query};
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, ResponseError};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use ad_buy_engine::chrono::Duration as ChronoDuration;
use ad_buy_engine::chrono::Utc;
use ad_buy_engine::constant::local_system_location::DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC;
use ad_buy_engine::constant::server_info::CAMPAIGN_SERVER_IP_PORT_TERSE;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::matrix::{MatrixData, MatrixValue};
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::diesel::PgConnection;
use ad_buy_engine::{Url, Uuid};
use diesel_migrations::run_pending_migrations;
use futures::executor;
use futures::FutureExt;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use r2d2_diesel::ConnectionManager;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{mpsc, Mutex};
use std::time::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn server() -> std::io::Result<()> {
    ad_buy_engine::dotenv::dotenv().ok();
    ad_buy_engine::env_logger::init();

    let (pool, cache, app_state, mem_store) = server_init_variables()?;

    let server = HttpServer::new(move || {
        App::new()
            // .wrap(RedirectSchemeBuilder::new().enable(true).build())
            .wrap(
                Cors::new()
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .data(JsonConfig::default().limit(4_096_000))
            .wrap(get_identity_service())
            .data(pool.clone())
            .data(cache.clone())
            .app_data(app_state.clone())
            .wrap_fn(|req, srv| {
                println!("\n");
                srv.call(req).map(|res| res)
            })
            .service(resource("/test_m").route(get().to(test_make)))
            .service(resource("/test_c").route(get().to(create_doc)))
            .service(resource("/test_r").route(get().to(restored_visit)))
            .service(resource("/test_u").route(get().to(upsert_doc)))
            .service(resource("/extra/{num}"))
            .service(resource("/extra/{num}").route(post().to(extra_multiple)))
            .service(resource("/extra").route(get().to(extra_single)))
            .service(resource("/learn/{campaign_id}").route(get().to(process_initial_click)))
            .service(resource("/action").route(post().to(action)))
            .configure(public_routes)
            .configure(private_routes)
            .service(
                scope("").default_service(
                    Files::new("", DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC)
                        .index_file("index.html")
                        .use_last_modified(true),
                ),
            )
    })
    .bind("campaign_server:80")?
    // .bind_openssl("campaign_server:443", ssl_config())?
    .workers(1)
    .run();

    server.await
}
use std::str::FromStr;

pub async fn upsert_doc(q: Query<HashMap<String, String>>) -> HttpResponse {
    match couch::restore_visit(
        q.get("db_name").expect("HT$R").clone(),
        q.get("visit_id").cloned().expect("FGDS"),
    )
    .await
    {
        Ok(mut res) => {
            println!("{:?}", &res);
            match couch::upsert(q.get("db_name").expect("HT$R").clone(), res).await {
                Ok(res) => {
                    println!("UPSERT SUCCESS\n\n");
                    HttpResponse::Ok().body("success upsert!")
                }
                Err(err) => {
                    println!("Error: {:?}", &err);
                    HttpResponse::InternalServerError().body("not upserted")
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", &err);
            HttpResponse::InternalServerError().body("not created")
        }
    }
}

pub async fn restored_visit(q: Query<HashMap<String, String>>) -> HttpResponse {
    match couch::restore_visit(
        q.get("db_name").expect("HT$R").clone(),
        q.get("visit_id").cloned().expect("^T$H"),
    )
    .await
    {
        Ok(res) => HttpResponse::Ok().body(format!("success restored visit {:?}", res)),
        Err(err) => {
            println!("Error: {:?}", &err);
            HttpResponse::Ok().body("could not restore visit")
        }
    }
}

pub async fn create_doc(q: Query<HashMap<String, String>>) -> HttpResponse {
    let v = Visit {
        _id: Utc::now().timestamp_millis().to_string(),
        _rev: "".to_string(),
        account_id: Uuid::new_v4(),
        campaign_id: Uuid::new_v4(),
        traffic_source_id: Uuid::new_v4(),
        funnel_id: None,
        impressions_from_traffic_source: 0,
        clicks: vec![],
        referrer: None,
        parameters: HashMap::new(),
        click_map: ClickMap {
            children: vec![],
            value: MatrixValue {
                id: Uuid::new_v4(),
                parent_matrix: None,
                group_idx: 0,
                item_idx: 0,
                depth: 0,
                data: MatrixData::Source,
            },
            seq_type: None,
            linked_conversion_id: None,
        },
        user_agent_data: UserAgentData {
            user_agent_string: "".to_string(),
            cpu_type: "".to_string(),
            device_name: "".to_string(),
            device_brand: "".to_string(),
            device_model: "".to_string(),
            rendering_engine_name: "".to_string(),
            rendering_engine_major: "".to_string(),
            rendering_engine_minor: "".to_string(),
            rendering_engine_patch: "".to_string(),
            os_name: "".to_string(),
            os_major: "".to_string(),
            os_minor: "".to_string(),
            os_patch: "".to_string(),
            os_patch_minor: "".to_string(),
            browser_name: "".to_string(),
            browser_major: "".to_string(),
            browser_minor: "".to_string(),
            browser_patch: "".to_string(),
        },
        geo_ip_data: GeoIPData {
            ip: IpAddr::from_str("24.245.77.178").expect("h64tdfg"),
            city: "".to_string(),
            continent: "".to_string(),
            country_iso_code: "".to_string(),
            subdivision_iso_code: "".to_string(),
            time_zone: "".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            metro_code: 0,
            postal_code: "".to_string(),
            asn: "".to_string(),
            isp: "".to_string(),
            connection_type: "".to_string(),
            is_anonymous_proxy: false,
            is_anonymous: false,
            is_anonymous_vpn: false,
            is_hosting_provider: false,
            is_public_proxy: false,
            is_satellite_provider: false,
            is_tor_exit_node: false,
            average_income: 0,
            population_density: 0,
        },
        conversions: vec![],
        custom_conversions: vec![],
        last_updated: Utc::now(),
    };
    println!("\nID\n{}\n", &v._id);

    match couch::insert_visit(q.get("db_name").expect("HT$R").clone(), v).await {
        Ok(res) => HttpResponse::Ok().body("CREATEd"),
        Err(err) => {
            println!("Error: {:?}", &err);
            HttpResponse::Ok().body("not created")
        }
    }
}

pub async fn test_make(q: Query<HashMap<String, String>>) -> HttpResponse {
    match create_couch_database(q.get("db_name").expect("HT$R").clone()).await {
        Ok(res) => HttpResponse::Ok().body("Healthy"),
        Err(err) => {
            println!("Error: {:?}", &err);
            HttpResponse::Ok().body("Created")
        }
    }
}

pub async fn couch_app_health() -> HttpResponse {
    let client = reqwest::Client::default();
    match client.get("http://couch_app:9000/health").send().await {
        Ok(res) => HttpResponse::Ok().body("Healthy"),
        Err(err) => {
            println!("Error: {:?}", &err);
            HttpResponse::Ok().body("ERROR")
        }
    }
}

pub fn server_init_variables() -> std::io::Result<(
    PgPool,
    Addr<RedisActor>,
    Data<Mutex<HashMap<Uuid, Campaign>>>,
    MemoryStore,
)> {
    let pool = establish_connection();
    diesel_migrations::run_pending_migrations(&pool.clone().get().expect("hyuu"))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let cache = if let Ok(var) = std::env::var("REDIS_URL") {
        let cache = RedisActor::start(&var);
        cache
    } else {
        panic!("Redis URL env var not found")
    };
    let res = load_click_identities_for_cache(&pool).expect("g56tFDS");
    println!("click identities number: {}", &res.len());
    res.into_iter().map(|s| create_click_identity(s, &cache));

    let mut filtered_restored: Vec<Campaign> = {
        use crate::schema::campaigns::dsl::campaigns;
        campaigns
            .load::<CampaignModel>(&pool.clone().get().expect("4rgfsadf"))
            .unwrap()
            .iter()
            .cloned()
            .map(|s| s.into())
            .collect::<Vec<Campaign>>()
    };

    filtered_restored.iter().filter(|s| {
        s.last_clicked.timestamp() < Utc::now().timestamp() + ChronoDuration::days(3).num_seconds()
    });
    println!("campagin in appstate: {}", &filtered_restored.len());

    let app_state = init_state(filtered_restored);
    let store = MemoryStore::new();
    Ok((pool, cache, app_state, store))
}
