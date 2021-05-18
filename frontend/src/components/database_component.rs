use crate::appstate::app_state::STATE;
use crate::database::errors::FrontendError;
use crate::database::js_pouchdb::bindings::create_pouch_database;
use crate::database::DatabaseInfo;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::future::LinkFuture;

pub struct DatabaseComponent {
    link: ComponentLink<Self>,
    db_info: DatabaseInfo,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
}

pub enum Msg {
    FetchDatabaseInfo,
    FetchDatabaseInfoDone(DatabaseInfo),
    SyncDatabaseDone(()),
    FetchDatabaseInfoFailed,
    SyncDatabaseFailed(FrontendError),
    SyncDatabase,
}

// async fn replicate_database(slim_account_id: String) -> Result<(), FrontendError> {
//     ConsoleService::info("DB: Replicating database");
//     let db = Database::new(slim_account_id.as_str());
//     db.replicate(slim_account_id).await
// }
//
// async fn fetch_db_info(slim_account_id: String) -> Result<DatabaseInfo, FrontendError> {
//     ConsoleService::info("Pouch Yew example: Fetching database info");
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

        Self {
            link,
            db_info: DatabaseInfo::new(slim_account_id),
            // props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SyncDatabaseDone(_) => {
                ConsoleService::info("DB: sync in progress");

                false
            }
            Msg::SyncDatabaseFailed(err) => {
                ConsoleService::info(&format!("DB Err: {}", err));
                false
            }

            Msg::SyncDatabase => {
                // create_pouch_database(self.db_info.db_name.clone());
                // let database_name = self.db_info.db_name.clone();
                // let future = async {
                //     match replicate_database(database_name).await {
                //         Err(err) => Msg::SyncDatabaseFailed(err),
                //         Ok(res) => Msg::SyncDatabaseDone(res),
                //     }
                // };
                // self.link.send_future(future);
                false
            }

            Msg::FetchDatabaseInfo => {
                create_pouch_database(self.db_info.db_name.clone());
                // let database_name = self.db_info.db_name.clone();
                // let future = async {
                //     match fetch_db_info(database_name).await {
                //         Ok(info) => Msg::FetchDatabaseInfoDone(info),
                //         Err(_) => Msg::FetchDatabaseInfoFailed,
                //     }
                // };
                //
                // self.link.send_future(future);
                false
            }
            Msg::FetchDatabaseInfoDone(info) => {
                ConsoleService::info("Pouch Yew example: Fetching database info done");
                self.db_info = info;
                true
            }
            Msg::FetchDatabaseInfoFailed => {
                ConsoleService::error("Pouch Yew example: Fetching database info failed");
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
            <p><button onclick=self.link.callback(|_| Msg::FetchDatabaseInfo)>{ "Get Database Info" }</button><i>{ format!("{:?}", self.db_info) }</i></p>
            <button onclick=self.link.callback(|_| Msg::SyncDatabase)>{ "Sync!" }</button>
        </div>
                }
    }
}

// #[wasm_bindgen(start)]
// pub fn run_app() {
//     App::<DatabaseComponent>::new().mount_to_body();
// }
