use serde::{Serialize, Deserialize};
use harsh::Harsh;

use super::{Episode, Season};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    pub id: String,
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnSeason {
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

impl From<Season> for ReturnSeason{
    fn from(season:Season) -> ReturnSeason{
        let hashid = create_hashid_client();
        ReturnSeason {
            id: hashid.encode(&[season.id as u64]),
            series_id: hashid.encode(&[season.series_id as u64]),
            slug: season.slug,
            title_en: season.title_en,
            title_ja: season.title_ja,
            title_romaji: season.title_romaji,
            keywords: season.keywords,
            anilist_id: season.anilist_id,
            description: season.description,
            synonyms: season.synonyms,
            episode_amt: season.episode_amt,
            episode_dur: season.episode_dur
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnEpisode {
    pub id: String,
    pub season_id: String,
    pub number: f64,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl From<&Episode> for ReturnEpisode{
    fn from(episode:&Episode) -> ReturnEpisode{
        let hashid = create_hashid_client();
        ReturnEpisode {
            id: hashid.encode(&[episode.id as u64]),
            season_id: hashid.encode(&[episode.season_id as u64]),
            number: episode.number,
            title: episode.title.clone(),
            description: episode.description.clone(),
        }
    }
}

fn create_hashid_client() -> Harsh {
    let hashid = Harsh::builder()
    .length(5)
    .salt("sakura-app!")
    .build()
    .unwrap();
    return hashid
}