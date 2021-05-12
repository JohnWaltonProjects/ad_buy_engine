use crate::api::campaign_server_api::click::process_initial_click;
use crate::api::campaign_server_api::extra_single::extra_single;
use crate::api::crud::click_identity::write::create_click_identity;
use crate::campaign_agent::CampaignAgent;
use crate::db::crud::click_identity::load_click_identities_for_cache;
use crate::helper_functions::{rate_limit, ssl_config};
use crate::private_routes::private_routes;
use crate::public_routes::public_routes;
use crate::utils::authentication::get_identity_service;
use crate::utils::cache::add_cache;
use crate::utils::config::CONFIG;
use crate::utils::database::establish_connection;
use crate::utils::middleware::click_processor::ClickProcessor;
use crate::utils::state::init_state;
use actix_cors::Cors;
use actix_files::Files;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_redis::RedisActor;
use actix_service::Service;
use actix_web::client::Client;
use actix_web::http::header;
use actix_web::http::header::HOST;
use actix_web::web::{get, post, resource, scope, service, Data, JsonConfig};
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use ad_buy_engine::constant::local_system_location::DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC;
use ad_buy_engine::constant::server_info::CAMPAIGN_SERVER_IP_PORT_TERSE;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use chrono::Duration as ChronoDuration;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel_migrations::run_pending_migrations;
use futures::executor;
use futures::FutureExt;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use r2d2_diesel::ConnectionManager;
use std::sync::mpsc;
use std::time::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

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
    let campaign_agent = CampaignAgent::start("campaign_server:1488");
    let couch = couch_rs::Client::new(
        "couch_database:5984",
        "couched_visits",
        "uX2b6@q5CxOjT7NrxYDc",
    )?;

    let server = HttpServer::new(move || {
        App::new()
            // .wrap(RedirectSchemeBuilder::new().enable(true).build())
            // .configure(add_cache)
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
            .data(Client::new())
            .data(pool.clone())
            .data(cache.clone())
            .data(couch.clone())
            .app_data(app_state.clone())
            .wrap_fn(|req, srv| {
                println!("\n");
                srv.call(req).map(|res| res)
            })
            .service(resource("/extra/{num}"))
            .service(resource("/extra").route(get().to(extra_single)))
            .service(resource("/learn/{campaign_id}").route(get().to(process_initial_click)))
            // .service(resource("/action").route(post().to(unimplemented!())))
            .configure(public_routes)
            .configure(private_routes)
            .service(
                scope("").default_service(
                    Files::new("", DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC)
                        .index_file("index.html")
                        .use_last_modified(true),
                ),
            )
            .data(campaign_agent.clone())
    })
    .bind("campaign_server:80")?
    // .bind_openssl("campaign_server:443", ssl_config())?
    .workers(1)
    .run();

    server.await
}
