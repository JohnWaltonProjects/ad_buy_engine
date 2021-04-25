use chrono::{DateTime, NaiveDateTime, Utc};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickEvent {
    pub timestamp: DateTime<Utc>,
    pub is_suspicious: bool,
    pub element_clicked: ClickableElement,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerseElement {
    pub element_id: Uuid,
    pub lander_url: Option<Url>,
    // pub group_idx: usize,
    // pub item_idx: usize,
    // pub depth: usize,
}

impl TerseElement {
    pub fn new(elem_id: Uuid, url: Option<Url>, gp_idx: usize, idx: usize, depth: usize) -> Self {
        Self {
            element_id: elem_id,
            lander_url: url,
            // group_idx: gp_idx,
            // item_idx: idx,
            // depth,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClickableElement {
    LandingPage(TerseElement),
    Offer(TerseElement),
}

impl ClickEvent {
    pub fn create(elem: ClickableElement) -> Self {
        Self {
            timestamp: Utc::now(),
            is_suspicious: false,
            element_clicked: elem,
        }
    }
}
