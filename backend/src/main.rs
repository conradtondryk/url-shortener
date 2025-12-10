use anyhow::Result;
use axum::Router;
use axum::routing::get;
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
use axum::extract::Path;

#[derive(Parser)]
struct LongUrl {
    long_url: Url,
}

struct UrlMapEntry {
    short_code: ShortCode,
    url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
struct ShortCode(String);

impl Display for ShortCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ShortCode {
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
struct UrlMap(HashMap<ShortCode, Url>);

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

    fn insert(&mut self, url_map_entry: UrlMapEntry) -> ShortCode {
        self.0
            .insert(url_map_entry.short_code.clone(), url_map_entry.url);
        url_map_entry.short_code
    }
}

async fn app() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() -> Result<()> {
    let LongUrl { long_url } = LongUrl::parse();
    let mut url_map = UrlMap::load()?;

    let short_code = url_map.insert(UrlMapEntry {
        url: long_url,
        short_code: ShortCode::new().clone(),
    });
    url_map.save()?;

    println!("Short URL: ctondryk.dev/{}", short_code);
    app().await.unwrap();
    Ok(())
}
