use anyhow::{Context, Result, anyhow};
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


#[allow(dead_code)]
#[derive(Deserialize)]
struct ValidInputs {
    num: u8,
    values: Vec<String>,
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

    pub fn filter(&self, key: &str, value: &str) -> Result<Quotes> {
        let url = format!("{}/quotes/{}?value={}", self.base_url, key, value);
        let response = get(url).context("Unable to fetch quotes from API")?;
        if response.status().is_success() {
            let result: Quotes = response
                .json()
                .context("Unable to parse response from API")?;
            Ok(result)
        } else {
            Err(anyhow!("Unsuccessful response from the API"))
        }
    }

    pub fn valid_inputs(&self, filter: &str) -> Result<Vec<String>> {
        let url = format!("{}/all/{}", self.base_url, filter);
        let response = get(url).context("Unable to fetch quotes from API")?;
        if response.status().is_success() {
            let result: ValidInputs = response
                .json()
                .context("Unable to parse response from API")?;
            Ok(result.values)
        } else {
            Err(anyhow!("Unsuccessful response from the API"))
        }
    }
}
