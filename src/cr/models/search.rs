use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrSearchResult {
    #[serde(rename = "__class__")]
    pub class: String,
    #[serde(rename = "__href__")]
    pub href: String,
    #[serde(rename = "__resource_key__")]
    pub resource_key: String,
    #[serde(rename = "__links__")]
    pub links: SearchResultLinks,
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    pub total: i64,
    pub items: Vec<SearchResultItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actions {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResultItem {
    #[serde(rename = "__class__")]
    pub class: String,
    #[serde(rename = "__href__")]
    pub href: String,
    #[serde(rename = "__resource_key__")]
    pub resource_key: String,
    #[serde(rename = "__links__")]
    pub links: SearchResultLinks,
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    #[serde(rename = "type")]
    pub item_type: String,
    pub total: i64,
    pub items: Vec<ItemItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemItem {
    #[serde(rename = "__actions__")]
    pub actions: Actions,
    #[serde(rename = "__class__")]
    pub class: Class,
    #[serde(rename = "__href__")]
    pub href: String,
    #[serde(rename = "__links__")]
    pub links: PurpleLinks,
    pub channel_id: ChannelId,
    pub description: String,
    pub external_id: String,
    pub id: String,
    pub images: Images,
    pub linked_resource_key: String,
    pub new: bool,
    pub new_content: bool,
    pub promo_description: String,
    pub promo_title: String,
    pub search_metadata: SearchMetadata,
    pub series_metadata: Option<SeriesMetadata>,
    pub slug: String,
    pub title: String,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub episode_metadata: Option<EpisodeMetadata>,
    pub playback: Option<String>,
    pub movie_listing_metadata: Option<MovieListingMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpisodeMetadata {
    pub ad_breaks: Option<Vec<AdBreak>>,
    pub availability_notes: String,
    pub available_offline: bool,
    pub duration_ms: i64,
    pub episode: String,
    pub episode_air_date: String,
    pub episode_number: Option<i64>,
    pub is_clip: bool,
    pub is_dubbed: bool,
    pub is_mature: bool,
    pub is_premium_only: bool,
    pub is_subbed: bool,
    pub mature_blocked: bool,
    pub maturity_ratings: Vec<String>,
    pub season_id: String,
    pub season_number: i64,
    pub season_title: String,
    pub sequence_number: f64,
    pub series_id: String,
    pub series_title: String,
    pub subtitle_locales: Vec<String>,
    pub tenant_categories: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdBreak {
    pub offset_ms: i64,
    #[serde(rename = "type")]
    pub ad_break_type: AdBreakType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub poster_tall: Option<Vec<Vec<PosterTall>>>,
    pub poster_wide: Option<Vec<Vec<PosterTall>>>,
    pub thumbnail: Option<Vec<Vec<PosterTall>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PosterTall {
    pub height: i64,
    pub source: String,
    #[serde(rename = "type")]
    pub poster_tall_type: PosterTallType,
    pub width: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleLinks {
    pub resource: Option<Continuation>,
    #[serde(rename = "resource/channel")]
    pub resource_channel: Option<Continuation>,
    #[serde(rename = "episode/season")]
    pub episode_season: Option<Continuation>,
    #[serde(rename = "episode/series")]
    pub episode_series: Option<Continuation>,
    pub streams: Option<Continuation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Continuation {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieListingMetadata {
    pub ad_breaks: Option<Vec<AdBreak>>,
    pub availability_notes: String,
    pub available_offline: bool,
    pub duration_ms: i64,
    pub first_movie_id: String,
    pub is_dubbed: bool,
    pub is_mature: bool,
    pub is_premium_only: bool,
    pub is_subbed: bool,
    pub mature_blocked: bool,
    pub maturity_ratings: Vec<String>,
    pub movie_release_year: i64,
    pub subtitle_locales: Vec<String>,
    pub tenant_categories: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchMetadata {
    pub score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeriesMetadata {
    pub availability_notes: String,
    pub episode_count: i64,
    pub is_dubbed: bool,
    pub is_mature: bool,
    pub is_simulcast: bool,
    pub is_subbed: bool,
    pub mature_blocked: bool,
    pub maturity_ratings: Vec<String>,
    pub season_count: i64,
    pub tenant_categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResultLinks {
    pub continuation: Option<Continuation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelId {
    #[serde(rename = "crunchyroll")]
    Crunchyroll,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Class {
    #[serde(rename = "panel")]
    Panel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdBreakType {
    #[serde(rename = "midroll")]
    Midroll,
    #[serde(rename = "preroll")]
    Preroll,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PosterTallType {
    #[serde(rename = "poster_tall")]
    PosterTall,
    #[serde(rename = "poster_wide")]
    PosterWide,
    #[serde(rename = "thumbnail")]
    Thumbnail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ItemType {
    #[serde(rename = "episode")]
    Episode,
    #[serde(rename = "movie_listing")]
    MovieListing,
    #[serde(rename = "series")]
    Series,
}