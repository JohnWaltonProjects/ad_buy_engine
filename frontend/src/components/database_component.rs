use crate::appstate::app_state::STATE;
use crate::database::errors::FrontendError;
use crate::database::{Database, DatabaseInfo};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::future::LinkFuture;

// #[derive(LinkFuture)]
pub struct DatabaseComponent {
    link: ComponentLink<Self>,
    db_info: DatabaseInfo,
    // props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: STATE,
}

pub enum Msg {
    FetchDatabaseInfo,
    FetchDatabaseInfoDone(DatabaseInfo),
    FetchDatabaseInfoFailed,
}

async fn fetch_db_info(account_id: String) -> Result<DatabaseInfo, FrontendError> {
    ConsoleService::info("Pouch Yew example: Fetching database info");
    let db = Database::new(account_id.as_str());
    db.info().await
}

impl Component for DatabaseComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let account_id = props.state.borrow().account.borrow().account_id.clone();

        Self {
            link,
            db_info: DatabaseInfo::new(&account_id),
            // props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchDatabaseInfo => {
                let database_name = self.db_info.db_name.clone();
                let future = async {
                    match fetch_db_info(database_name).await {
                        Ok(info) => Msg::FetchDatabaseInfoDone(info),
                        Err(_) => Msg::FetchDatabaseInfoFailed,
                    }
                };

                self.link.send_future(future);
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
                    <p><b>{ format!("{} (v{})", "Yew & Pouch", crate::database::version()) }</b></p>
                    <button onclick=self.link.callback(|_| Msg::FetchDatabaseInfo)>{ "Get Database Info" }</button>
                    <p><i>{ format!("{:?}", self.db_info) }</i></p>
        </div>
                }
    }
}

// #[wasm_bindgen(start)]
// pub fn run_app() {
//     App::<DatabaseComponent>::new().mount_to_body();
// }
