use anyhow::Result;
use clap::Parser;
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::ErrorKind;
use url::Url;
const FILE_PATH: &str = "urls.json";

#[derive(Deserialize, Serialize, Parser)]
struct CliInput {
    url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
struct ShortUrl(String);

impl Display for ShortUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
struct UrlMap(HashMap<ShortUrl, Url>);

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

    fn insert(&mut self, long_url: Url) -> ShortUrl {
        let short_url = ShortUrl::new();
        self.0.insert(short_url.clone(), long_url);
        short_url
    }
}

fn main() -> Result<()> {
    let CliInput { url } = CliInput::parse();
    let mut url_map = UrlMap::load()?;

    let short_url = url_map.insert(url);
    url_map.save()?;

    println!("Short URL: ctondryk.dev/{}", short_url);
    Ok(())
}
