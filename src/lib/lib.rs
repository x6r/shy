use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

pub mod command;
mod format;
mod glyphs;
mod helper;
pub mod log;
pub mod queue;

// todo: autodetect the port (%appdata%/MusicBee/WWWServerconfig.xml)
static MUSICBEE_REST_URL: &str = "http://localhost:8080";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct NowPlaying {
    #[serde(rename = "Album")]
    album: String,
    #[serde(rename = "Artist")]
    artist: String,
    #[serde(rename = "Title")]
    title: String,
    position: u32,
    duration: u32,
    file: String,
    playing: String,
    queued: bool,
    repeat: String,
    scrobbling: bool,
    shuffle: bool,
    volume: f32,
}

#[derive(Debug)]
pub enum ShuffleStatus {
    On,
    Off,
    Toggle,
}

impl From<bool> for ShuffleStatus {
    fn from(b: bool) -> Self {
        match b {
            true => ShuffleStatus::On,
            false => ShuffleStatus::Off,
        }
    }
}

impl From<&String> for ShuffleStatus {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "on" => ShuffleStatus::On,
            "off" => ShuffleStatus::Off,
            _ => ShuffleStatus::Toggle,
        }
    }
}

impl NowPlaying {
    async fn new() -> Result<NowPlaying, Box<dyn Error>> {
        let body = Client::new()
            .get(format::url("NP"))
            .send()
            .await?
            .text()
            .await?;
        let np: NowPlaying = serde_json::from_str(&body)?;

        Ok(np)
    }
}
