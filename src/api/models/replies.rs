use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    pub id: String,
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub id: String,
    pub series_id: String,
    pub slug: String,
    pub title_en: Option<String>,
    pub title_ja: Option<String>,
    pub title_romaji: String,
    pub keywords: Option<String>,
    pub anilist_id: Option<i32>,
    pub description: Option<String>,
    pub synonyms: Option<String>,
    pub episode_amt: Option<i32>,
    pub episode_dur: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub id: String,
    pub season_id: String,
    pub number: f64,
    pub title: Option<String>,
    pub description: Option<String>,
}