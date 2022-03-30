use chrono::NaiveDateTime;
use harsh::Harsh;
use reqwest::Url;
use rocket::{
    response::status::{Forbidden, NoContent},
    State,
};
use rocket_contrib::{json, json::JsonValue};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    api::models::{replies, Episode, Media, Season},
    cr::CrunchyrollClient,
};

pub mod anilist;
pub mod cr;
pub mod models;

#[rocket::get("/")]
pub async fn index() -> &'static str {
    "api_v0"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestReturn {
    series: String,
    seasons: Vec<TestSeasons>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestSeasons {
    title: String,
    episodes: Vec<f32>,
}

#[rocket::get("/season/<slug>")]
pub async fn get_series<'r>(
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    _cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
    _hashid: State<'_, Harsh>,
    slug: String,
) -> Result<JsonValue, NoContent> {
    match query_as!(
        Season,
        r#"
        select id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur from "season"
        where slug = $1
    "#,
    slug,
    )
    .fetch_one(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => {
            let cvt: replies::ReturnSeason = r.into();
            return Ok(json!(cvt))},
        Err(_) => return Err(NoContent),
    };
}

#[rocket::get("/episodes?<season>")]
pub async fn get_episodes<'r>(
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    _cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
    hashid: State<'_, Harsh>,
    season: String,
) -> Result<JsonValue, Forbidden<String>> {
    let id = match hashid.decode(season) {
        Ok(e) => e,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    match query_as!(
        Episode,
        r#"
        select id, season_id, number, title, cr_id, description from "episode"
        where season_id = $1
        order by number asc;
    "#,
        id[0] as i32,
    )
    .fetch_all(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => {
            let mut ret: Vec<replies::ReturnEpisode> = vec![];
            r.iter().for_each(|f| {
                let cvt:replies::ReturnEpisode = f.into();
                ret.push(cvt)
            });
            return Ok(json!(ret));
        }
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };
}

#[rocket::get("/search?<q>")]
pub async fn search<'r>(
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    _cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
    _hashid: State<'_, Harsh>,
    q: String,
) -> Result<JsonValue, Forbidden<String>> {
    match query_as!(
        Season,
        r#"
        select id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur from "season"
        where LOWER(title_en) like '%' || $1 || '%'
        or LOWER(title_romaji) like '%' || $1 || '%'
        or LOWER(slug) like '%' || $1 || '%'
        or LOWER(synonyms) like '%' || $1 || '%'
    "#,
    q,
    )
    .fetch_all(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(season) => {
            dbg!(season.clone());
            let mut return_seasons:Vec<replies::ReturnSeason> = vec![];
            for r in season {
                let cvt: replies::ReturnSeason = r.into();
            return_seasons.push(cvt)
            }

            return Ok(json!(return_seasons))},
        Err(_) => return Err(Forbidden(Some("oop".to_string()))),
    };
}

#[rocket::get("/showexperience/<id>")]
pub async fn show_experience<'r>(
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
    hashid: State<'_, Harsh>,
    id: String,
) -> Result<JsonValue, Forbidden<String>> {
    let hid = match hashid.decode(id) {
        Ok(e) => e,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    //check if we have any experiences
    match query_as!(
        Media,
        r#"
        select id, episode_id, host, quality, sub_lang, sub_burned, sub_url, url, time from "media"
        where episode_id = $1
    "#,
        hid[0] as i32,
    )
    .fetch_all(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => {
            // if we do just return the data
            if r.len() > 0 {
                return Ok(json!(r));
            } else {
                // otherwise (assuming crunchyroll) we generate the video URL
                let mut cr = cr_rw.write().await;
                let episode = match query!(
                    r#"
                    select id, season_id, cr_id from episode
                    where id = $1"#,
                    hid[0] as i32
                )
                .fetch_one(&mut pool.acquire().await.unwrap())
                .await
                {
                    Ok(e) => e,
                    Err(_) => todo!(),
                };
                let season = match query!(
                    r#"
                    select id, cr_id from season
                    where id = $1
                    "#,
                    episode.season_id
                )
                .fetch_one(&mut pool.acquire().await.unwrap())
                .await
                {
                    Ok(e) => e,
                    Err(e) => return Err(Forbidden(Some(e.to_string()))),
                };
                let ep_list = match query_as!(
                    Episode,
                    r#"
                    select id, season_id, number, title, cr_id, description from "episode"
                    where season_id = $1
                "#,
                    season.id,
                )
                .fetch_all(&mut pool.acquire().await.unwrap())
                .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(Forbidden(Some(e.to_string()))),
                };
                let season_cr_id = season.cr_id.unwrap();
                let mut all_episodes = match cr.episodes(season_cr_id.clone(), 1).await {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(Forbidden(Some(
                            "Episode list could not be fetched from external source.".to_string(),
                        )))
                    }
                };
                if all_episodes.items.iter().any(|epi| epi.is_premium_only == true || epi.mature_blocked == true) {
                    println!("Premium or mature content detected, using account 2");
                    all_episodes = match cr.episodes(season_cr_id, 2).await {
                        Ok(i) => i,
                        Err(e) => {
                            return Err(Forbidden(Some(
                                "Episode list could not be fetched from the external source.".to_string() + &e.to_string(),
                            )))
                        }
                    };
                }
                let mut media_list: Vec<Media> = vec![];
                if all_episodes.items.len() > ep_list.len() {
                    // reindex all episodes that were fetched
                }
                for e in all_episodes.items.iter() {
                    let stream_array = match cr.stream(e.to_owned()).await {
                        Ok(i) => i,
                        Err(e) => {
                            return Err(Forbidden(Some(
                                "Stream url could not be fetched: ".to_string() + &e.to_string(),
                            )))
                        }
                    };
                    let cr_url = match stream_array.streams.adaptive_hls {
                        Some(e) => match e.en_us {
                            Some(e) => e,
                            None => {
                                return Err(Forbidden(Some(
                                    "There are no English subtitles.".to_string(),
                                )))
                            }
                        },
                        None => {
                            return Err(Forbidden(Some(
                                "Expiry for stream URL could not be parsed.".to_string(),
                            )))
                        }
                    };
                    let time = match Url::parse(&cr_url.url) {
                        Ok(e) => NaiveDateTime::from_timestamp(
                            match e.query() {
                                Some(g) => g.to_string()[8..18].parse::<i64>().unwrap(),
                                None => {
                                    return Err(Forbidden(Some(
                                        "Expiry for stream URL could not be parsed.".to_string(),
                                    )))
                                }
                            },
                            0,
                        ),
                        Err(_) => {
                            return Err(Forbidden(Some(
                                "Expiry for stream URL could not be parsed.".to_string(),
                            )))
                        }
                    };
                    media_list.push(Media {
                        id: 1,
                        episode_id: ep_list[media_list.len()].id,
                        host: 1,
                        quality: "adaptive".to_string(),
                        sub_lang: Some(
                            format!(
                                "{}-{}",
                                &format!("{:?}", cr_url.hardsub_locale)[..2],
                                &format!("{:?}", cr_url.hardsub_locale)[2..]
                            )
                            .to_lowercase(),
                        ),
                        sub_burned: true,
                        sub_url: None,
                        url: cr_url.url,
                        time,
                    })
                }
                // TODO: save this in the db
                return Ok(json!(media_list));
            }
        }
        // we can't access the db!!! this is bad!!
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };
}
