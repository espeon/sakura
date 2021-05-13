use std::sync::Arc;

use cr::CrunchyrollClient;
use reqwest::Client as ReqwestClient;
use tokio::sync::RwLock;

extern crate serde_json;

mod db;
mod cr;

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let req_client = ReqwestClient::new();
    let pool = db::get_pool().await?;

    let mut client = CrunchyrollClient::new(pool.clone(), req_client).await?;
    
    let client_cover = Arc::new(tokio::sync::RwLock::new(client));

    //let search_result = &client.search("higehiro".to_string()).await?;
    
    //let season = &client.seasons(search_result.items[0].items[0].id.to_owned()).await?;

   // let episodes = client.episodes(season.items[0].clone().id).await?;

    //let stream = client.stream(episodes.items[1].clone()).await?;
    
    // start up our web server
    // dunno if i want this in a separate thread or not
    dbg!("help");
    go(pool, client_cover).await?;
    Ok(())
}

async fn go(pool: sqlx::Pool<sqlx::Postgres>, crcl: Arc<RwLock<CrunchyrollClient>>) -> anyhow::Result<()> {
    rocket::build()
        .manage(pool)
        .manage(crcl)
        .mount("/api", rocket::routes![api::index])
        .mount("/api", rocket::routes![api::test])
        .launch()
        .await?;
    Ok(())
}

#[rocket::get("/")]
async fn hello() -> &'static str {
    "hello world!"
}