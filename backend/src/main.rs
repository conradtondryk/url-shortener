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
    // TODO: why is this of type CliInput. Seems completely wrong. I don't think you should mix the
    // arg parsing and actual data storage types.
    long_url: CliInput,
}

// TODO: think you should do a longer name like UrlPairs
// TODO: don't think Vec is the right datastructure. you are mostly gonna wanna do short -> long
// lookups so thats a hashmap
#[derive(Deserialize, Serialize)]
struct Pairs(Vec<UrlPair>);

impl Pairs {
    fn load() -> Result<Self> {
        // TODO: make filepath a const so you don't type it out twice
        match fs::read_to_string("urls.json") {
            // TODO: you dont need map_err(Into::into) when using anyhow I don't think. Would also
            // be good to add .context on this too.
            Ok(data) => serde_json::from_str(&data).map_err(Into::into),
            // TODO: you wanna do a Pairs::new fn instead of manually creating the vector inside
            // the tuple struct.
            // Calling code shouldnt have to care how Pairs is storing its data internally.
            // It also makes your intention more clear / makes it easier to read if you do
            // something like Pairs::new or Pairs::empty
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Pairs(vec![])),
            Err(e) => Err(e.into()),
        }
    }
    fn save(&self) -> Result<()> {
        // TODO: I'd probs put the file create on a separate line and make a var for it
        // It's a bit more readable imo even if it takes more space
        serde_json::to_writer(File::create("urls.json")?, &self.0)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let CliInput { url } = CliInput::parse();
    let mut list = Pairs::load()?;
    // TODO: this is shoulds probs be a method on Pairs.
    list.0.push(UrlPair {
        // TODO: you wanna abstract this logic away somewhere.
        // probably make a UrlPair::new method which generates the shorturl for you.
        // main fn shouldnt have much low level logic in it
        short_url: format!("ctondryk.dev/{}", rand::thread_rng().gen_range(0..1000000)),
        // TODO: again, really weird to be storing this as CliInput
        long_url: CliInput { url },
    });
    list.save()?;
    Ok(())
}
