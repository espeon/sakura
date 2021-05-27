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
    api::models::{replies, Episode, Media, Season, Series},
    cr::CrunchyrollClient,
};

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
#[rocket::get("/test")]
pub async fn test<'r>(
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
) -> Result<JsonValue, Forbidden<String>> {
    dbg!("help");
    let mut cr = cr_rw.write().await;
    let sr = match cr.search("hitoribocchi".to_string()).await {
        Ok(r) => r,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    let mut r = TestReturn {
        series: sr.items[0].items[0].title.clone(),
        seasons: vec![],
    };

    let seasons = match cr.seasons(sr.items[0].items[0].id.to_owned()).await {
        Ok(r) => r,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    let series = match query_as!(
        Series,
        r#"
        insert into series(slug, title, cr_id)
        values($1, $2, $3)
        returning id, slug, title, cr_id
    "#,
        slug::slugify(seasons.items[0].to_owned().title),
        seasons.items[0].title,
        seasons.items[0].series_id
    )
    .fetch_one(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => r,
        Err(_) => {
            match query_as!(
                Series,
                r#"
            select id, slug, title, cr_id from "series"
            where slug = $1
        "#,
                slug::slugify(seasons.items[0].to_owned().title),
            )
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            {
                Ok(i) => i,
                Err(e) => return Err(Forbidden(Some(e.to_string()))),
            }
        }
    };

    for season in seasons.items {
        r.seasons.push(TestSeasons {
            title: season.title.clone(),
            episodes: vec![],
        });
        let c_season = r.seasons.len();
        let season_res = match query_as!(
            Season,
            r#"
            insert into season(series_id, slug, title_romaji, cr_id)
            values($1, $2, $3, $4)
            returning id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur
        "#,
        series.id,
        slug::slugify(season.title.to_owned()),
        season.title,
        season.id.clone()
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
        slug::slugify(season.title.to_owned()),
            )
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            {
                Ok(i) => i,
                Err(e) => return Err(Forbidden(Some(e.to_string()))),
            }}
        };

        let episodes = match cr.to_owned().episodes(season.id.clone()).await {
            Ok(r) => r,
            Err(e) => return Err(Forbidden(Some(e.to_string()))),
        };

        for e in episodes.items.iter() {
            dbg!(e);
            r.seasons[c_season - 1]
                .episodes
                .push(e.sequence_number as f32);
            match query_as!(
                Episode,
                r#"
                insert into episode(season_id, number, title, cr_id, description)
                values($1, $2, $3, $4, $5)
            "#,
                season_res.id,
                e.sequence_number as f32,
                e.title,
                e.id,
                e.description
            )
            .fetch_all(&mut pool.acquire().await.unwrap())
            .await
            {
                Ok(_r) => (),
                Err(e) => {
                    dbg!(e);
                    continue;
                }
            };
        }
    }

    Ok(json!(r))
}

#[rocket::get("/season/<slug>")]
pub async fn get_series_test<'r>(
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    _cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
    hashid: State<'_, Harsh>,
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
            let ret = replies::Season {
                id: hashid.encode(&[r.id as u64]),
                series_id: hashid.encode(&[r.series_id as u64]),
                slug: r.slug,
                title_en: r.title_en,
                title_ja: r.title_ja,
                title_romaji: r.title_romaji,
                keywords: r.keywords,
                anilist_id: r.anilist_id,
                description: r.description,
                synonyms: r.synonyms,
                episode_amt: r.episode_amt,
                episode_dur: r.episode_dur
            };
            return Ok(json!(ret))},
        Err(_) => return Err(NoContent),
    };
}

#[rocket::get("/episodes?<season>")]
pub async fn get_episodes_test<'r>(
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
    "#,
        id[0] as i32,
    )
    .fetch_all(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => {
            let mut ret: Vec<replies::Episode> = vec![];
            r.iter().for_each(|f| {
                ret.push(replies::Episode {
                    id: hashid.encode(&[f.id as u64]),
                    season_id: hashid.encode(&[f.season_id as u64]),
                    number: f.number,
                    title: f.title.clone(),
                    description: f.description.clone(),
                })
            });
            return Ok(json!(ret));
        }
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
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
    "#,
        id[0] as i32,
    )
    .fetch_all(&mut pool.acquire().await.unwrap())
    .await
    {
        Ok(r) => {
            let mut ret: Vec<replies::Episode> = vec![];
            r.iter().for_each(|f| {
                ret.push(replies::Episode {
                    id: hashid.encode(&[f.id as u64]),
                    season_id: hashid.encode(&[f.season_id as u64]),
                    number: f.number,
                    title: f.title.clone(),
                    description: f.description.clone(),
                })
            });
            return Ok(json!(ret));
        }
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
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
        // if we do just return the data
        Ok(r) => {
            if r.len() > 0 {
                return Ok(json!(r));
            } else {
                // otherwise (assuming crunchyroll) we generate the video URL
                let mut cr = cr_rw.write().await;
                let season = match query!(
                    r#"
                    select id, cr_id from season"#
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
                let all_episodes = match cr.episodes(season.cr_id.unwrap()).await {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(Forbidden(Some(
                            "Episode list could not be fetched from external source.".to_string(),
                        )))
                    }
                };
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
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };
}
