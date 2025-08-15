use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ApiResponse {
    pub data: ApiData,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ApiData {
    pub list: Vec<Radio>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Radio {
    pub ID: u32,
    pub name: String,
    pub URL: String,
    pub image: RadioImage,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RadioImage {
    pub prefix: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RadioDetailsResponse {
    pub data: RadioDetailsData,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RadioDetailsData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub timeline: Option<Vec<RadioSongInfo>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RadioSongInfo {
    pub artistTitle: Option<String>,
    pub songTitle: Option<String>,
    pub image: Option<RadioImage>,
    pub remainingSeconds: Option<u32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RadioStreamData {
    pub data: RadioStreamDataItem,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct RadioStreamDataItem {
    pub URL: String,
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct RadioData {
    pub stream_url: String,
    pub logo_url: String,
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
}
