use reqwest::Client;
use serde_json::json;

use self::queries::{GET_SEASON,SEARCH_SEASON};

use self::models::{AnilistSeason, AnilistSeasonResult};

mod models;
mod queries;

pub async fn search_season(query: String) -> anyhow::Result<Vec<AnilistSeasonResult>, anyhow::Error> {
    println!("anilist-season");
    let json = json!(
            {
            "query": SEARCH_SEASON,
            "variables": {
                "search": query,
                "page": 1,
                "perPage": 3
            }
        }
    );
    let season: AnilistSeason = Client::new()
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await?
        .json()
        .await?;
        dbg!(season.clone());
        let results = season.data.anime.results;
    Ok(results)
}

pub async fn get_season(id: i32) -> anyhow::Result<Vec<AnilistSeasonResult>, anyhow::Error> {
    let json = json!(
            {
            "query": GET_SEASON,
            "variables": {
                "id": id,
                "isAdult": true
            }
        }
    );
    let season: AnilistSeason = Client::new()
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await?
        .json()
        .await?;
        let results = season.data.anime.results;
    Ok(results)
}
