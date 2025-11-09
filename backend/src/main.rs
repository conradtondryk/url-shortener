use anyhow::Result;
use clap::Parser;
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::ErrorKind;

const FILE_PATH: &str = "urls.json";

#[derive(Deserialize, Serialize, Parser)]
struct CliInput {
    url: String,
}

struct ShortUrl(String);

impl ShortUrl {
    fn new() -> Self {
        Self(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect::<String>(),
        )
    }
}
#[derive(Deserialize, Serialize)]
struct UrlMap(HashMap<String, String>);

impl UrlMap {
    fn load() -> Result<Self> {
        match fs::read_to_string(FILE_PATH) {
            Ok(data) => serde_json::from_str(&data).map_err(Into::into),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(UrlMap(HashMap::new())),
            Err(e) => Err(e.into()),
        }
    }

    fn save(&self) -> Result<()> {
        serde_json::to_writer_pretty(File::create(FILE_PATH)?, &self.0)?;
        Ok(())
    }

    fn insert(&mut self, short_url: String, long_url: String) {
        self.0.insert(short_url, long_url);
    }
}

fn main() -> Result<()> {
    let CliInput { url } = CliInput::parse();
    let mut url_map = UrlMap::load()?;

    let short_url = ShortUrl::new();
    url_map.insert(short_url.0.clone(), url);
    url_map.save()?;
    println!("Short URL: ctondryk.dev/{}", short_url.0);
    Ok(())
}
