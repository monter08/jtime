use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::string::ToString;

const DEFAULT_NAGER_URL: &str = "https://date.nager.at";
const DEFAULT_NAGER_COUNTRY_CODE: &str = "PL";

pub struct Nager {
    client: Client,
    url: String,
    country_code: String,
}

#[derive(Deserialize)]
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
        let response = self
            .client
            .get(self.build_url(format!(
                "/api/v3/PublicHolidays/{}/{}",
                year, self.country_code
            )))
            .send()?;

        if response.status().is_success() {
            Ok(response.json()?)
        } else {
            Err(anyhow::anyhow!("Failed to fetch holidays"))
        }
    }
}
