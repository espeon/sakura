use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnilistSeason {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub anime: Anime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anime {
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub results: Vec<AnilistSeasonResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageInfo {
    pub total: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnilistSeasonResult {
    pub id: i64,
    pub title: Title,
    pub synonyms: Vec<String>,
    #[serde(rename = "coverImage")]
    pub cover_image: CoverImage,
    #[serde(rename = "type")]
    pub result_type: String,
    pub format: String,
    pub description: Option<String>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    #[serde(rename = "isLicensed")]
    pub is_licensed: bool,
    pub season: Option<String>,
    #[serde(rename = "seasonYear")]
    pub season_year: Option<i64>,
    #[serde(rename = "startDate")]
    pub start_date: StartDate,
    pub duration: Option<i32>,
    pub episodes: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StartDate {
    pub year: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Title {
    pub romaji: String,
    pub english: Option<String>,
    pub native: String,
}