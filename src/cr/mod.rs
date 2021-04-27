use chrono::Duration;
use reqwest::Client;
use std::ops::Add;

use self::models::{Cms, CrApiAccessToken, CrApiCms, search::CrSearchResult, seasons::CrSeriesResult};

pub mod models;

#[derive(Debug, Clone)]
pub struct CrAccessToken {
    pub access_token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub refresh_token: Option<String>,
}
pub enum AccessLevel {
    Basic,
    Standard,
    Premium,
}

#[derive(Clone, Debug)]
pub struct CrunchyrollClient {
    pub token: CrAccessToken,
    pub cms: Option<Cms>,
    db: sqlx::Pool<sqlx::Sqlite>,
    pub req: Client,
}

impl CrunchyrollClient {
    pub async fn new(
        db: sqlx::Pool<sqlx::Sqlite>,
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
    pub async fn seasons(&self, id: String) -> anyhow::Result<CrSeriesResult, anyhow::Error> {
        let info = self.clone();
        let cms = &self.check_cms().await?;
        let res: CrSeriesResult = info
            .req
            .get(format!("https://beta-api.crunchyroll.com/cms/v2{}/seasons",cms.bucket))
            .query(&[("series_id", id), ("Signature", cms.signature.clone()), ("Policy", cms.policy.clone()), ("Key-Pair-Id", cms.key_pair_id.clone())])
            .bearer_auth(info.token.access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    pub async fn check_cms(&self) -> anyhow::Result<Cms, anyhow::Error>{
        match &self.cms {
            Some(cms) => return Ok(cms.to_owned()),
            None => {
                let res: CrApiCms = self
                .req
                .get("https://beta-api.crunchyroll.com/index/v2")
                .bearer_auth(&self.token.access_token)
                .send()
                .await?
                .json()
                .await?;

                let ret = res.cms;

                return Ok(ret)
            }
        }
    }
    pub async fn access_level(&self) -> AccessLevel {
        if self.token.refresh_token == None {
            match self.cms {
                Some(_) => return AccessLevel::Standard,
                None => return AccessLevel::Basic
            }
        } else {
            return AccessLevel::Premium;
        }
    }
}
