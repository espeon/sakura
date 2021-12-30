use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use std::ops::Add;

use crate::cr::models::episodes::CrEpisodesResult;

use self::models::{
    episodes::EpisodeItem, search::CrSearchResult, seasons::CrSeasonsResult,
    series::CrSeriesResult, stream::CrStreamResult, Cms, CrApiAccessToken, CrApiCms,
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
    pub token2: Option<CrAccessToken>,
    pub cms: Option<Cms>,
    pub cms2: Option<Cms>,
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
            .header("authorization", "Basic Y3Jfd2ViOg==") // used for webapp
            .header("content-type", "application/x-www-form-urlencoded")
            .form(&[("grant_type", "client_id")]) //cr uses both a form and basic auth
            .send()
            .await?
            .json()
            .await?;

        let token_length = Duration::minutes(res.expires_in);
        let expiry = chrono::Utc::now().add(token_length);

        let client = CrunchyrollClient {
            token: CrAccessToken {
                access_token: res.access_token, //fetched token
                expiry, // usually three hours but we use returned value anyways
                refresh_token: None, // non logged-in user does not include a refresh token
            },
            token2: None,
            cms: None,
            cms2: None,
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
            token2: None,
            cms: None,
            cms2: None,
            db,
            req,
        };
        Ok(client)
    }
    pub async fn add_credentials(
        &mut self,
        username: String,
        password: String,
    ) -> anyhow::Result<CrAccessToken, anyhow::Error> {
        let res: CrApiAccessToken = self
            .req
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

        if res.refresh_token == None {
            return Err(anyhow::anyhow!("The second set of credentials *needs* to be premium"));
        }
        let token_object = CrAccessToken {
            // insert token info ()
            access_token: res.access_token,
            expiry,
            refresh_token: res.refresh_token,
        };

        self.token2 = Some(token_object.clone());

        Ok(token_object)
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
        let cms = self.check_cms(1).await?;
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
        let cms = self.check_cms(1).await?;
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
        which_cms: i8,
    ) -> anyhow::Result<CrEpisodesResult, anyhow::Error> {
        println!("episode");
        let info = self.clone();
        let cms = self.check_cms(which_cms).await?;
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
            if self.token2.is_none() {
                return Err(anyhow::anyhow!("Premium only"));
            }
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
    pub async fn check_cms(&mut self, which: i8) -> anyhow::Result<Cms, anyhow::Error> {
        let which_cms = match which {
            2 => &self.cms2,
            _ => &self.cms
        };
        let cms_info = match which_cms {
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
                    let cms_info = self.get_cms(which).await?;
                    cms_info
                }
            }
            None => {
                let cms_info = self.get_cms(which).await?;
                cms_info
            }
        };
        match which {
            2 => self.cms2 = Some(cms_info.clone()),
            _ => self.cms = Some(cms_info.clone())
        };
        //self.cms = Some(cms_info.clone());
        Ok(cms_info)
    }
    async fn get_cms(&self, which:i8) -> anyhow::Result<Cms, anyhow::Error> {
        let which_token = match which {
            2 => match &self.token2 {
                Some(e) => e,
                None => todo!(),
            },
            _ => &self.token
        };
        println!("Getting CMS Info");
        let res: CrApiCms = self
            .req
            .get("https://beta-api.crunchyroll.com/index/v2")
            .bearer_auth(which_token.access_token.clone())
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
