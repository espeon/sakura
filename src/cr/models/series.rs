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
    pub links: Links,
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub slug: String,
    pub slug_title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub season_tags: Vec<String>,
    pub images: Images,
    pub maturity_ratings: Vec<String>,
    pub episode_count: i64,
    pub season_count: i64,
    pub media_count: i64,
    pub content_provider: String,
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
pub struct Actions {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub poster_tall: Vec<Vec<Poster>>,
    pub poster_wide: Vec<Vec<Poster>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poster {
    pub width: i64,
    pub height: i64,
    #[serde(rename = "type")]
    pub poster_type: Type,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Links {
    #[serde(rename = "series/channel")]
    pub series_channel: Series,
    #[serde(rename = "series/seasons")]
    pub series_seasons: Series,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    #[serde(rename = "poster_tall")]
    PosterTall,
    #[serde(rename = "poster_wide")]
    PosterWide,
}