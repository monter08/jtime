use crate::cache::Cache;
use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::ToString;

const DEFAULT_NAGER_URL: &str = "https://date.nager.at";
const DEFAULT_NAGER_COUNTRY_CODE: &str = "PL";

pub struct Nager {
    client: Client,
    url: String,
    country_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct NagerHoliday {
    pub date: String,
    #[serde(rename = "localName")]
    pub local_name: String,
    // Api returns also this fields, but i don't see any use for therm
    // pub name: String,
    // #[serde(rename = "countryCode")]
    // pub country_code: String,
    // pub fixed: bool,
    // pub global: bool,
    // pub counties: Option<String>,
    // #[serde(rename = "launchYear")]
    // pub launch_year: Option<String>,
    // pub types: Vec<String>,
}
pub type HolidayMap = HashMap<String, String>;

impl Nager {
    pub fn new(url: Option<String>, country_code: Option<String>) -> Self {
        Nager {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .expect("Failed to build client"),
            url: url.unwrap_or_else(|| DEFAULT_NAGER_URL.to_string()),
            country_code: country_code.unwrap_or_else(|| DEFAULT_NAGER_COUNTRY_CODE.to_string()),
        }
    }

    fn build_url<S: AsRef<str>>(&self, path: S) -> String {
        format!("{}{}", self.url, path.as_ref())
    }

    pub fn get_all_holidays(&self, year: String) -> Result<Vec<NagerHoliday>> {
        let cache = Cache::new(format!("nager_holidays_{}", year).to_string());
        if let Some(data) = cache.load()? {
            return Ok(serde_json::from_str(&data)?);
        }
        let response = self
            .client
            .get(self.build_url(format!(
                "/api/v3/PublicHolidays/{}/{}",
                year, self.country_code
            )))
            .send()?;

        if response.status().is_success() {
            let holidays: Vec<NagerHoliday> = response.json()?;
            cache.save(&serde_json::to_string(&holidays)?)?;
            Ok(holidays)
        } else {
            Err(anyhow::anyhow!("Failed to fetch holidays"))
        }
    }

    pub fn get_all_holidays_map(&self, year: String) -> Result<HolidayMap> {
        let holidays = self.get_all_holidays(year)?;
        let mut holidays_map = HolidayMap::new();
        for holiday in holidays {
            holidays_map.insert(holiday.date.clone(), holiday.local_name);
        }
        Ok(holidays_map)
    }
}
