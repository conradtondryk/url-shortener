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
use std::sync::{Arc, Mutex};
use url::Url;
const FILE_PATH: &str = "urls.json";
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Redirect;
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

async fn start_server(shared_map: Arc<Mutex<UrlMap>>) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/:short_code", get(redirect_handler))
        .with_state(shared_map);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
async fn redirect_handler(
    Path(short_code): Path<String>,
    State(url_map): State<Arc<Mutex<UrlMap>>>,
) -> Result<Redirect, (StatusCode, String)> {
    let map = url_map.lock().unwrap();

    if let Some(long_url) = map.0.get(&ShortCode(short_code)) {
        Ok(Redirect::permanent(long_url.as_str()))
    } else {
        Err((StatusCode::NOT_FOUND, "Short link not found".to_string()))
    }
}

async fn root() -> &'static str {
    "Hello, this is the root page! Enter your short URL in the browser to be redirected to the original URL."
}

#[tokio::main]
async fn main() -> Result<()> {
    let LongUrl { long_url } = LongUrl::parse();
    let mut url_map = UrlMap::load()?;

    let short_code = url_map.insert(UrlMapEntry {
        short_code: ShortCode::new().clone(),
        url: long_url,
    });
    url_map.save()?;

    let shared_map = Arc::new(Mutex::new(url_map));
    println!("Short URL: http://localhost:3000/{}", short_code);
    start_server(shared_map).await?;

    Ok(())
}
