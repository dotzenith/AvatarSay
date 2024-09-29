use anyhow::{Context, Result};
use reqwest::blocking::get;
use serde::Deserialize;
use std::env;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct QuotesInner {
    pub quote: String,
    pub character: String,
    pub nation: String,
    pub bending: String,
    pub episode: String,
    pub book: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Quotes {
    pub num: u16,
    pub quotes: Vec<QuotesInner>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct APIManager {
    base_url: String,
}

impl APIManager {
    pub fn new() -> Result<Self> {
        let url = env::var("AvatarAPIBaseURL").context("AvatarAPIBaseURL not found")?;
        Ok(APIManager { base_url: url })
    }

    pub fn random(&self) -> Result<Quotes> {
        let url = format!("{}/quotes", self.base_url);
        let result: Quotes = get(url)
            .context("Unable to fetch quotes from API")?
            .json()
            .context("Unable to parse response from API")?;
        Ok(result)
    }
}
