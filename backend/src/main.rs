use anyhow::Result;
use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::ErrorKind;

#[derive(Deserialize, Serialize, Parser)]
struct CliInput {
    url: String,
}

#[derive(Deserialize, Serialize)]
struct UrlPair {
    short_url: String,
    long_url: CliInput,
}

#[derive(Deserialize, Serialize)]
struct Pairs(Vec<UrlPair>);

impl Pairs {
    fn load() -> Result<Self> {
        match fs::read_to_string("urls.json") {
            Ok(data) => serde_json::from_str(&data).map_err(Into::into),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Pairs(vec![])),
            Err(e) => Err(e.into()),
        }
    }
    fn save(&self) -> Result<()> {
        serde_json::to_writer(File::create("urls.json")?, &self.0)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let CliInput { url } = CliInput::parse();
    let mut list = Pairs::load()?;
    list.0.push(UrlPair {
        short_url: format!("ctondryk.dev/{}", rand::thread_rng().gen_range(0..1000000)),
        long_url: CliInput { url },
    });
    list.save()?;
    Ok(())
}
