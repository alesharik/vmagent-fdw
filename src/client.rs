use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum TargetHealth {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down
}

impl Display for TargetHealth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetHealth::Up => write!(f, "up"),
            TargetHealth::Down => write!(f, "down")
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Target {
    #[serde(rename = "discoveredLabels")]
    pub discovered_labels: HashMap<String, String>,
    #[serde(rename = "labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "scrapePool")]
    pub scrape_pool: String,
    #[serde(rename = "scrapeUrl")]
    pub scrape_url: String,
    #[serde(rename = "lastError")]
    pub last_error: String,
    #[serde(rename = "lastScrape")]
    pub last_scrape: Option<DateTime<Utc>>,
    #[serde(rename = "lastScrapeDuration")]
    pub last_scrape_duration: f64,
    #[serde(rename = "lastSamplesScraped")]
    pub last_samples_scraped: u64,
    #[serde(rename = "health")]
    pub health: TargetHealth,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TargetList {
    #[serde(rename = "activeTargets")]
    pub active: Vec<Target>,
    #[serde(rename = "droppedTargets")]
    pub dropped: Vec<Target>,
}

#[derive(Deserialize, Clone, Debug)]
struct Response {
    data: TargetList,
}

pub struct Client {
    client: reqwest::blocking::Client,
    addr: String,
}

impl Client {
    pub fn from_str(address: &str) -> Client {
        Client {
            client: reqwest::blocking::Client::new(),
            addr: address.to_string(),
        }
    }

    pub fn targets(&self) -> crate::error::Result<TargetList> {
        Ok(self.client.get(format!("{}/api/v1/targets", &self.addr))
            .send()?
            .json::<Response>()?
            .data)
    }
}