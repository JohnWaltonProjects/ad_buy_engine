use crate::appstate::app_state::STATE;
// use crate::utils::pouchdb::database::Database;
// use crate::utils::pouchdb::errors::Error as FrontendError;
// use crate::utils::pouchdb::types::DatabaseInfo;

use crate::utils::javascript::js_bindings::{log, replicate};
// use crate::utils::pouchdb_rs::PouchDB;
use ad_buy_engine::serde_json::Value;
use pouchdb::options::replication::Replication;
use pouchdb::{PouchDB, PouchDBOrStringRef};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::future::LinkFuture;

pub struct DatabaseComponent {
    link: ComponentLink<Self>,
    local_database: pouchdb::PouchDB,
    remote_database: pouchdb::PouchDB,
    database_name: String,
    // db_info: DatabaseInfo,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
}

pub enum Msg {
    FetchDatabaseInfo,
    Destroy,
    Test,
    // FetchDatabaseInfoDone(DatabaseInfo),
    SyncDatabaseDone(()),
    FetchDatabaseInfoFailed,
    // SyncDatabaseFailed(FrontendError),
    SyncDatabase,
    Ignore,
}

// async fn destroy_db() {
//     let db = PouchDB::new("testdb1234");
//     let res = db.destroy().await;
//     log(&format!("{:?}", res));
// }

// pub async fn test(slim_account_id: String) {
//     // let local_db = PouchDB::new("testdb1234");
//     // let remote_db = PouchDB::new(&format!("http://127.0.0.1:8081/visits/{}", slim_account_id));
//     // let opts = Replication::default();
//     // replic
//
// }

// async fn replicate_database(slim_account_id: String) -> Result<(), FrontendError> {
//     ConsoleService::info("DB: Replicating pouchdb");
//     // crate::utils::pouchdb_rs
//     test_r
//     // let db = Database::new(slim_account_id.as_str());
//     // db.replicate(slim_account_id).await
// }

// async fn fetch_db_info(slim_account_id: String) -> Result<DatabaseInfo, FrontendError> {
//     ConsoleService::info("Pouch Yew example: Fetching pouchdb info");
//     let db = Database::new(slim_account_id.as_str());
//     db.info().await
// }

impl Component for DatabaseComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let slim_account_id = props
            .state
            .borrow()
            .account
            .borrow()
            .account_id
            .to_string()
            .chars()
            .filter(|s| *s != '-')
            .collect::<String>();

        let remote_database = PouchDB::new(&format!(
            "http://127.0.0.1:8081/visits/{}",
            &slim_account_id
        ));

        Self {
            link,
            local_database: pouchdb::PouchDB::new(slim_account_id.clone()),
            remote_database,
            database_name: slim_account_id,
            // db_info: DatabaseInfo::new(slim_account_id),
            // props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Test => {
                log("test...");
                let local = PouchDBOrStringRef::PouchDB(&self.local_database);
                let remote = PouchDBOrStringRef::PouchDB(&self.remote_database);
                let options = Replication::default();

                let sync_res = pouchdb::PouchDB::replicate(remote, local, &options, true);
                match sync_res {
                    Ok(res) => {
                        log("Ok");
                    }
                    Err(err) => {
                        log("Err..");
                        log(&err.to_string())
                    }
                }
                false
            }

            Msg::Ignore => {
                log("ignore...");
                false
            }

            Msg::SyncDatabaseDone(_) => {
                ConsoleService::info("DB: sync in progress");

                false
            }

            Msg::Destroy => {
                // let future = async {
                //     let res = destroy_db().await;
                //     Msg::Ignore
                // };
                //
                // self.link.send_future(future);
                false
            }

            // Msg::SyncDatabaseFailed(err) => {
            //     ConsoleService::info(&format!("DB Err: {}", err));
            //     false
            // }
            Msg::SyncDatabase => {
                // let res = replicate(slim_account_id);
                false
            }

            Msg::FetchDatabaseInfo => {
                // let name = self.db_info.db_name.clone();
                // let future = async {
                //     match fetch_db_info(name).await {
                //         Ok(info) => Msg::FetchDatabaseInfoDone(info),
                //         Err(_) => Msg::FetchDatabaseInfoFailed,
                //     }
                // };
                //
                // self.link.send_future(future);
                false
            }
            // Msg::FetchDatabaseInfoDone(info) => {
            //     ConsoleService::info("Pouch Yew example: Fetching pouchdb info done");
            // self.db_info = info;
            // true
            // }
            Msg::FetchDatabaseInfoFailed => {
                ConsoleService::error("Pouch Yew example: Fetching pouchdb info failed");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div>
            // <p><button onclick=self.link.callback(|_| Msg::FetchDatabaseInfo)>{ "Get Database Info" }</button><i>{ format!("{:?}", self.db_info) }</i></p>
            // <p><button onclick=self.link.callback(|_| Msg::Destroy)>{ "Destroy" }</button></p>
            // <p><button onclick=self.link.callback(|_| Msg::SyncDatabase)>{ "Sync" }</button></p>
            <p><button onclick=self.link.callback(|_| Msg::Test)>{ "Test" }</button></p>
        </div>
                }
    }
}

// #[wasm_bindgen(start)]
// pub fn run_app() {
//     App::<DatabaseComponent>::new().mount_to_body();
// }
