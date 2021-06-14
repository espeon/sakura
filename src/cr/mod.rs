use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use std::ops::Add;

use crate::cr::models::episodes::CrEpisodesResult;

use self::models::{
    episodes::EpisodeItem, search::CrSearchResult, seasons::CrSeasonsResult,
    series::CrSeriesResult, stream::CrStreamResult, 
    Cms, CrApiAccessToken, CrApiCms,
};

pub mod models;

#[derive(Debug, Clone)]
pub struct CrAccessToken {
    pub access_token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub refresh_token: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessLevel {
    Basic,
    Standard,
    Premium,
}

#[derive(Clone, Debug)]
pub struct CrunchyrollClient {
    pub token: CrAccessToken,
    pub cms: Option<Cms>,
    db: sqlx::Pool<sqlx::Postgres>,
    pub req: Client,
}

#[allow(dead_code)]
impl CrunchyrollClient {
    pub async fn new(
        db: sqlx::Pool<sqlx::Postgres>,
        req: Client,
    ) -> anyhow::Result<Self, anyhow::Error> {
        let res: CrApiAccessToken = req
            .post("https://beta-api.crunchyroll.com/auth/v1/token")
            .header("authorization", "Basic Y3Jfd2ViOg==")
            .header("content-type", "application/x-www-form-urlencoded")
            .form(&[("grant_type", "client_id")])
            .send()
            .await?
            .json()
            .await?;

        let token_length = Duration::minutes(res.expires_in);
        let expiry = chrono::Utc::now().add(token_length);

        let client = CrunchyrollClient {
            token: CrAccessToken {
                access_token: res.access_token, //fetched token
                expiry,
                refresh_token: None,
            },
            cms: None,
            db,
            req,
        };
        Ok(client)
    }
    pub async fn new_with_credentials(
        db: sqlx::Pool<sqlx::Postgres>,
        req: Client,
        username: String,
        password: String,
    ) -> anyhow::Result<Self, anyhow::Error> {
        let res: CrApiAccessToken = req
            .post("https://beta-api.crunchyroll.com/auth/v1/token")
            .header(
                "authorization",
                "Basic MWlhZ2ZsbjAycF9yY2R3amxzZ2E6MWl2dk85eVdubDUxTEd5N2VGTm5fdVdmMVluSUNGNEE=",
            ) // used for Android client
            .header("content-type", "application/x-www-form-urlencoded")
            .form(&[
                ("grant_type", "password"),
                ("username", &username),
                ("password", &password),
                ("scope", "offline_access"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let token_length = Duration::minutes(res.expires_in);
        let expiry = chrono::Utc::now().add(token_length);

        let client = CrunchyrollClient {
            token: CrAccessToken {
                // insert token info (refresh token included if premium)
                access_token: res.access_token,
                expiry,
                refresh_token: res.refresh_token,
            },
            cms: None,
            db,
            req,
        };
        Ok(client)
    }
    pub async fn search(&self, to_search: String) -> anyhow::Result<CrSearchResult, anyhow::Error> {
        let res: CrSearchResult = self
            .req
            .get("https://beta-api.crunchyroll.com/content/v1/search")
            .query(&[("q", to_search), ("n", "8".to_string())])
            .bearer_auth(self.token.access_token.to_owned())
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    pub async fn series(
        &mut self,
        series_id: String,
    ) -> anyhow::Result<CrSeriesResult, anyhow::Error> {
        println!("series");
        let info = self.clone();
        let cms = self.check_cms().await?;
        let res: CrSeriesResult = info
            .req
            .get(format!(
                "https://beta-api.crunchyroll.com/cms/v2{}/series/{}",
                cms.bucket, series_id
            ))
            .query(&[
                // i haaaaaaaaaaaaate crunchyroll's new api because of this
                ("Signature", cms.signature.clone()),
                ("Policy", cms.policy.clone()),
                ("Key-Pair-Id", cms.key_pair_id.clone()),
            ])
            .bearer_auth(info.token.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    pub async fn seasons(
        &mut self,
        series_id: String,
    ) -> anyhow::Result<CrSeasonsResult, anyhow::Error> {
        println!("season");
        let info = self.clone();
        let cms = self.check_cms().await?;
        let res: CrSeasonsResult = info
            .req
            .get(format!(
                "https://beta-api.crunchyroll.com/cms/v2{}/seasons",
                cms.bucket
            ))
            .query(&[
                // i haaaaaaaaaaaaate crunchyroll's new api because of this
                ("series_id", series_id),
                ("Signature", cms.signature.clone()),
                ("Policy", cms.policy.clone()),
                ("Key-Pair-Id", cms.key_pair_id.clone()),
            ])
            .bearer_auth(info.token.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    pub async fn episodes(
        &mut self,
        season_id: String,
    ) -> anyhow::Result<CrEpisodesResult, anyhow::Error> {
        println!("episode");
        let info = self.clone();
        let cms = self.check_cms().await?;
        let res: CrEpisodesResult = info
            .req
            .get(format!(
                "https://beta-api.crunchyroll.com/cms/v2{}/episodes",
                cms.bucket
            ))
            .query(&[
                ("season_id", season_id),
                ("Signature", cms.signature.clone()),
                ("Policy", cms.policy.clone()),
                ("Key-Pair-Id", cms.key_pair_id.clone()),
            ])
            .bearer_auth(info.token.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    pub async fn stream(
        &mut self,
        episode: EpisodeItem,
    ) -> anyhow::Result<CrStreamResult, anyhow::Error> {
        let info = self.clone();
        if episode.is_premium_only && self.access_level().await != AccessLevel::Premium {
            return Err(anyhow::anyhow!("Premium only"));
        }
        let res: CrStreamResult = info
            .req
            .get(episode.playback.unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    pub async fn check_cms(&mut self) -> anyhow::Result<Cms, anyhow::Error> {
        let cms_info = match &self.cms {
            Some(cms) => {
                println!("Cms info found");

                let now = chrono::Utc::now();
                if now < cms.expires.parse::<DateTime<Utc>>()? {
                    println!("{:?}", &cms);
                    println!("not expired");
                    cms.to_owned()
                } else {
                    println!("expired!");
                    println!(
                        "it is currently {} and token expires at {}",
                        now, cms.expires
                    );
                    let cms_info = self.get_cms().await?;
                    cms_info
                }
            }
            None => {
                let cms_info = self.get_cms().await?;
                cms_info
            }
        };
        self.cms = Some(cms_info.clone());
        Ok(cms_info)
    }
    async fn get_cms(&self) -> anyhow::Result<Cms, anyhow::Error> {
        println!("Getting CMS Info");
        let res: CrApiCms = self
            .req
            .get("https://beta-api.crunchyroll.com/index/v2")
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .json()
            .await?;

        println!("{:?}", &res.cms);
        Ok(res.cms)
    }
    pub async fn access_level(&self) -> AccessLevel {
        if self.token.refresh_token == None {
            match self.cms {
                Some(_) => return AccessLevel::Standard,
                None => return AccessLevel::Basic,
            }
        } else {
            return AccessLevel::Premium;
        }
    }
}
