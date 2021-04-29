use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrStreamResult {
    pub audio_locale: String,
    pub subtitles: HashMap<String, SubtitleData>,
    pub streams: StreamOption,
    #[serde(rename = "QoS")]
    pub qos: QoS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QoS {
    pub region: String,
    #[serde(rename = "cloudFrontRequestId")]
    pub cloud_front_request_id: String,
    #[serde(rename = "lambdaRunTime")]
    pub lambda_run_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamData {
    pub hardsub_locale: Locale,
    pub url: String,
    pub vcodec: Vcodec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubtitleData {
    pub locale: Locale,
    pub url: String,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Vcodec {
    #[serde(rename = "h264")]
    H264,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOption {
    pub adaptive_dash: Option<StreamLocale>,
    pub adaptive_hls: Option<StreamLocale>,
    pub download_hls: Option<StreamLocale>,
    pub drm_adaptive_dash: Option<StreamLocale>,
    pub drm_adaptive_hls: Option<StreamLocale>,
    pub drm_download_hls: Option<StreamLocale>,
    pub drm_multitrack_adaptive_hls_v2: Option<StreamLocale>,
    pub multitrack_adaptive_hls_v2: Option<StreamLocale>,
    pub urls: Option<StreamLocale>,
    pub vo_adaptive_dash: Option<StreamLocale>,
    pub vo_adaptive_hls: Option<StreamLocale>,
    pub vo_drm_adaptive_dash: Option<StreamLocale>,
    pub vo_drm_adaptive_hls: Option<StreamLocale>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamLocale {
    #[serde(rename = "")]
    pub unsubbed: Option<StreamData>,
    #[serde(rename = "ar-ME")]
    pub ar_me: Option<StreamData>,
    #[serde(rename = "de-DE")]
    pub de_de: Option<StreamData>,
    #[serde(rename = "en-US")]
    pub en_us: Option<StreamData>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<StreamData>,
    #[serde(rename = "es-LA")]
    pub es_la: Option<StreamData>,
    #[serde(rename = "fr-FR")]
    pub fr_fr: Option<StreamData>,
    #[serde(rename = "it-IT")]
    pub it_it: Option<StreamData>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<StreamData>,
    #[serde(rename = "ru-RU")]
    pub ru_ru: Option<StreamData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Locale {
    #[serde(rename = "")]
    None,
    #[serde(rename = "ar-ME")]
    ArMe,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-LA")]
    EsLa,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "ru-RU")]
    RuRu,
}
