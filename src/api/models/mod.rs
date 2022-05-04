
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

pub mod replies;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub cr_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub id: i32,
    pub series_id: i32,
    pub slug: String,
    pub title_en: Option<String>,
    pub title_ja: Option<String>,
    pub title_romaji: String,
    pub cr_id: Option<String>,
    pub keywords: Option<String>,
    pub anilist_id: Option<i32>,
    pub description: Option<String>,
    pub synonyms: Option<String>,
    pub episode_amt: Option<i32>,
    pub episode_dur: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub id: i32,
    pub season_id: i32,
    pub number: f64,
    pub title: Option<String>,
    pub cr_id: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    pub id: i32,
    pub episode_id: i32,
    pub host: i32,
    pub quality: String,
    pub sub_lang: Option<String>,
    pub sub_burned: bool,
    pub sub_url: Option<String>,
    pub url: String,
    pub time: NaiveDateTime,
}