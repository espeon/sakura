use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrSeriesResult {
    #[serde(rename = "__class__")]
    pub class: String,
    #[serde(rename = "__href__")]
    pub href: String,
    #[serde(rename = "__resource_key__")]
    pub resource_key: String,
    #[serde(rename = "__links__")]
    pub links: Actions,
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    pub total: i64,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actions {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "__class__")]
    pub class: String,
    #[serde(rename = "__href__")]
    pub href: String,
    #[serde(rename = "__resource_key__")]
    pub resource_key: String,
    #[serde(rename = "__links__")]
    pub links: Links,
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub series_id: String,
    pub season_number: i64,
    pub is_complete: bool,
    pub description: String,
    pub keywords: Vec<Option<serde_json::Value>>,
    pub season_tags: Vec<String>,
    pub images: Actions,
    pub is_mature: bool,
    pub mature_blocked: bool,
    pub is_subbed: bool,
    pub is_dubbed: bool,
    pub is_simulcast: bool,
    pub seo_title: String,
    pub seo_description: String,
    pub availability_notes: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Links {
    #[serde(rename = "season/channel")]
    pub season_channel: Season,
    #[serde(rename = "season/episodes")]
    pub season_episodes: Season,
    #[serde(rename = "season/series")]
    pub season_series: Season,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub href: String,
}
