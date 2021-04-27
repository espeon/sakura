use serde::{Serialize, Deserialize};
pub mod search;
pub mod seasons;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrApiCms {
    pub cms: Cms,
    pub service_available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cms {
    pub bucket: String,
    pub policy: String,
    pub signature: String,
    pub key_pair_id: String,
    pub expires: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrApiAccessToken {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub country: String,
}
