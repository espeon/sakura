use harsh::Harsh;
use reqwest::Client;
use serde_json::json;

use rocket::{response::status::Forbidden, State};
use rocket_contrib::{json as rjson, json::JsonValue};
use sqlx::query_as;

use crate::{
    api::{
        anilist,
        models::Season,
    },
};

use self::queries::{GET_SEASON, SEARCH_SEASON};

use self::models::{AnilistSeason, AnilistSeasonResult};

mod models;
mod queries;

pub async fn search_season(
    query: String,
) -> anyhow::Result<Vec<AnilistSeasonResult>, anyhow::Error> {
    
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

    dbg!(&json);
    let seasona = Client::new()
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await?;
    let season: AnilistSeason = seasona.json().await?;
    dbg!(season.clone());
    let results = season.data.anime.results;
    println!("anilist-season");
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


#[rocket::get("/episodes?<season>")]
pub async fn get_metadata<'r>(
    season: String,
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    hashid: State<'_, Harsh>,
) -> Result<JsonValue, Forbidden<String>> {

    let id = match hashid.decode(season) {
        Ok(e) => e,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    let season = match query_as!(
        Season,
        r#"
        select id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur from "season"
        where id = $1
    "#,
    id[0] as i32,
    )
    .fetch_one(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    let anilist_season = match anilist::search_season(season.title_romaji.clone()).await {
        Ok(e) => e,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };
    //insert into season(slug, title_en, title_ja, title_romaji, cr_id, anilist_id, description, synonyms, episode_amt, episode_dur)
    let season_res = match query_as!(
            Season,
            r#"
            update season
            set slug = $1,
                title_en = $2,
                title_ja = $3,
                title_romaji = $4,
                anilist_id = $5,
                description = $6,
                synonyms = $7,
                episode_amt = $8,
                episode_dur = $9
            where id = $10
            returning id, series_id, slug, title_en, title_ja, title_romaji, keywords, cr_id, anilist_id, description, synonyms, episode_amt, episode_dur
        "#,
        slug::slugify(anilist_season[0].title.romaji.clone()),
        anilist_season[0].title.english,
        anilist_season[0].title.native,
        anilist_season[0].title.romaji,
        anilist_season[0].id as i32,
        anilist_season[0].description,
        anilist_season[0].synonyms.join(",.,"),
        anilist_season[0].episodes,
        anilist_season[0].duration,
        id[0] as i32
        )
        .fetch_one(&mut pool.acquire().await.unwrap())
        .await
        {
            Ok(r) => r,
            Err(_) => {
                match query_as!(
                Season,
                r#"
            select id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur from "season"
            where slug = $1
        "#,
        slug::slugify(season.title_romaji.to_owned()),
            )
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            {
                Ok(i) => i,
                Err(e) => return Err(Forbidden(Some(e.to_string()))),
            }}
        };

        Ok(rjson!(season_res))
}
