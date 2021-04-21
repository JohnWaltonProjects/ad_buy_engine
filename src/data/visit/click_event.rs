use chrono::NaiveDateTime;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickEvent {
    pub timestamp: NaiveDateTime,
    pub is_suspicious: bool,
    pub element_clicked: ClickableElement,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerseElement {
    pub element_id: Uuid,
    pub lander_url: Option<Url>,
    pub group_idx: usize,
    pub item_idx: usize,
    pub depth: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClickableElement {
    LandingPage(TerseElement),
    Offer(TerseElement),
}
