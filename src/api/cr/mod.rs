use rocket::{response::status::Forbidden, State};
use rocket_contrib::{json, json::JsonValue};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    api::{
        anilist,
        models::{Season, Series},
    },
    cr::CrunchyrollClient,
};

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
#[rocket::get("/series?<cr_id>")] //&<anilist_id>
pub async fn index_series<'r>(
    cr_id: String,
    //anilist_id: String,
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
) -> Result<JsonValue, Forbidden<String>> {
    dbg!("help");
    let mut cr = cr_rw.write().await;

    //let anilist_ids:Vec<i32> = vec![];
    //anilist_id.split(",").for_each(|id| anilist_ids.push(match id.parse::<i32>(){
    //    Ok(e) => e,
    //    Err(_) => 0 as i32,
    //}));
    //
    let series = match cr.series(cr_id).await {
        Ok(r) => r,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    let mut r = TestReturn {
        series: series.title,
        seasons: vec![],
    };

    let seasons = match cr.seasons(series.id.to_owned()).await {
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
        if season.title.clone().contains("Dub") {
            println!("dub detected!!!!")
        } else {
            let anilist_season = match anilist::search_season(season.title.clone()).await {
                Ok(e) => e,
                Err(e) => return Err(Forbidden(Some(e.to_string()))),
            };
            r.seasons.push(TestSeasons {
                title: slug::slugify(anilist_season[0].title.romaji.clone()),
                episodes: vec![],
            });
            let c_season = r.seasons.len();
            let season_res = match query_as!(
            Season,
            r#"
            insert into season(series_id, slug, title_en, title_ja, title_romaji, cr_id, anilist_id, description, synonyms, episode_amt, episode_dur)
            values($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            returning id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur
        "#,
        series.id,
        slug::slugify(anilist_season[0].title.romaji.clone()),
        anilist_season[0].title.english,
        anilist_season[0].title.native,
        anilist_season[0].title.romaji,
        season.id.clone(),
        anilist_season[0].id as i32,
        anilist_season[0].description,
        anilist_season[0].synonyms.join(",.,"),
        anilist_season[0].episodes,
        anilist_season[0].duration,
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

            let episodes = match cr.to_owned().episodes(season.id.clone(), 1).await {
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
    }

    Ok(json!(r))
}

#[rocket::get("/episodes?<cr_id>")]
pub async fn index_episodes<'r>(
    cr_id: String,
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
    cr_rw: State<'r, Arc<RwLock<CrunchyrollClient>>>,
) -> Result<JsonValue, Forbidden<String>> {
    let mut r: Vec<f32> = vec![];

    let season_db = match query_as!(Season,r#"
    select id, series_id, slug, title_en, title_ja, title_romaji, cr_id, keywords, anilist_id, description, synonyms, episode_amt, episode_dur from "season"
    where cr_id = $1
"#,cr_id).fetch_all(&mut pool.acquire().await.unwrap()).await{
        Ok(e) => e,
        Err(_) => todo!(),
    };

    let cr = cr_rw.write().await;
    let episodes = match cr.to_owned().episodes(cr_id, 1).await {
        Ok(r) => r,
        Err(e) => return Err(Forbidden(Some(e.to_string()))),
    };

    for e in episodes.items.iter() {
        dbg!(e);
        r.push(e.sequence_number as f32);
        match query_as!(
            Episode,
            r#"
            insert into episode(season_id, number, title, cr_id, description)
            values($1, $2, $3, $4, $5)
        "#,
            season_db[0].id,
            e.sequence_number as f32,
            e.title,
            e.id,
            e.description
        )
        .fetch_one(&mut pool.acquire().await.unwrap())
        .await
        {
            Ok(_r) => (),
            Err(e) => {
                dbg!(e);
                continue;
            }
        };
    }
    Ok(json!(r))
}
