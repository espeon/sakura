use cr::CrunchyrollClient;
use reqwest::Client as ReqwestClient;

extern crate serde_json;

mod db;
mod cr;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let req_client = ReqwestClient::new();
    let pool = db::get_pool().await?;
    let client = CrunchyrollClient::new(pool.clone(), req_client).await?;
    let search_result = &client.search("fruits basket".to_string()).await?;
    println!("{} - {}",search_result.items[0].items[0].title, search_result.items[0].items[0].id);
    let season = &client.seasons(search_result.items[0].items[0].id.to_owned()).await?;

    season.items.clone().into_iter().for_each(|a| println!("{}. {} - {}",a.season_number, a.title, a.id));
    
    // start up our web server
    // dunno if i want this in a separate thread or not
    //rocket(pool, client).await?;
    Ok(())
}

async fn _rocket(pool: sqlx::Pool<sqlx::Sqlite>, crcl: CrunchyrollClient) -> anyhow::Result<()> {
    rocket::build()
        .manage(pool)
        .manage(crcl)
        .mount("/", rocket::routes![hello])
        .launch()
        .await?;
    Ok(())
}

#[rocket::get("/")]
async fn hello() -> &'static str {
    "hello world!"
}