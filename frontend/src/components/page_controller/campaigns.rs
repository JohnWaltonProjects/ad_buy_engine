use crate::appstate::app_state::AppState;
use crate::components::tab_state::ActivatedTab;
use crate::notify_primary;
use crate::utils::routes::AppRoute;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;

pub enum Msg {
    Click,
    Ignore,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: Rc<RefCell<AppState>>,
}

pub struct CampaignBtn {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    active: bool,
}

impl Component for CampaignBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Ignore));
        let active_tab = state_clone!(props.state).borrow().return_app_route();
        let active = tab_is_active!(AppRoute::Campaign, active_tab);

        Self {
            link,
            router,
            props,
            active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.active = true;
                self.props
                    .state
                    .borrow()
                    .selected_elements
                    .borrow_mut()
                    .clear();
                self.props
                    .state
                    .borrow_mut()
                    .set_first_prime_column_and_reset_other_columns_and_save_to_browser_and_set_route(AppRoute::Campaign);
                self.router.send(ChangeRoute(AppRoute::Campaign.into()))
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let active_tab = state_clone!(props.state).borrow().return_app_route();
        self.active = tab_is_active!(AppRoute::Campaign, active_tab);
        true
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|_| Msg::Click);
        let a_class = if self.active {
            "active-tab uk-active uk-display-block"
        } else {
            "uk-display-block"
        };
        let icon_class = if self.active {
            "active-tab fa fa-rocket uk-display-block uk-text-center"
        } else {
            "fa fa-rocket uk-display-block uk-text-center"
        };
        html! {
        <li onclick=callback>
            <span class=icon_class></span>
            <a class=a_class>{"Campaigns"}</a>
        </li>
        }
    }
}
