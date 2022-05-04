use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrEpisodesResult {
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
    pub items: Vec<EpisodeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actions {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpisodeItem {
    #[serde(rename = "__class__")]
    pub class: Class,
    #[serde(rename = "__href__")]
    pub href: String,
    #[serde(rename = "__resource_key__")]
    pub resource_key: String,
    #[serde(rename = "__links__")]
    pub links: Links,
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    pub id: String,
    pub channel_id: ChannelId,
    pub series_id: String,
    pub series_title: String,
    pub season_id: String,
    pub season_title: String,
    pub season_number: i64,
    pub episode: String,
    pub episode_number: Option<i64>,
    pub sequence_number: f64,
    pub production_episode_id: String,
    pub title: String,
    pub description: String,
    pub next_episode_id: Option<String>,
    pub next_episode_title: Option<String>,
    pub hd_flag: bool,
    pub is_mature: bool,
    pub mature_blocked: bool,
    pub episode_air_date: String,
    pub is_subbed: bool,
    pub is_dubbed: bool,
    pub is_clip: bool,
    pub seo_title: String,
    pub seo_description: String,
    pub season_tags: Vec<String>,
    pub available_offline: bool,
    pub media_type: Class,
    pub slug: String,
    pub images: Images,
    pub duration_ms: i64,
    pub ad_breaks: Option<Vec<AdBreak>>,
    pub is_premium_only: bool,
    pub listing_id: String,
    pub subtitle_locales: Vec<String>,
    pub playback: Option<String>,
    pub availability_notes: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdBreak {
    #[serde(rename = "type")]
    pub ad_break_type: AdBreakType,
    pub offset_ms: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub thumbnail: Vec<Vec<Thumbnail>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thumbnail {
    pub width: i64,
    pub height: i64,
    #[serde(rename = "type")]
    pub thumbnail_type: ThumbnailType,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Links {
    pub ads: Option<Ads>,
    #[serde(rename = "episode/channel")]
    pub episode_channel: Ads,
    #[serde(rename = "episode/next_episode")]
    pub episode_next_episode: Option<Ads>,
    #[serde(rename = "episode/season")]
    pub episode_season: Option<Ads>,
    #[serde(rename = "episode/series")]
    pub episode_series: Ads,
    pub streams: Option<Ads>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ads {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdBreakType {
    #[serde(rename = "midroll")]
    Midroll,
    #[serde(rename = "preroll")]
    Preroll,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelId {
    #[serde(rename = "crunchyroll")]
    Crunchyroll,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Class {
    #[serde(rename = "episode")]
    Episode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThumbnailType {
    #[serde(rename = "thumbnail")]
    Thumbnail,
}