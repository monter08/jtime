use crate::{
    api::API,
    cache::Cache,
    config::Config,
    view::{helper::Helper, Calendar, Render},
};
use anyhow::Result;

use chrono::{Datelike, Utc};

pub fn execute(config: &Config, api: &API, use_cache: &bool, month: &Option<u32>) -> Result<()> {
    let cache = Cache::new("month".to_string());

    if *use_cache {
        if let Some(data) = cache.load() {
            println!("{}", data);
            return Ok(());
        }
    }

    let month = month.unwrap_or_else(|| Utc::now().month());
    let range = Calendar::range_days_for_month(Utc::now().year(), month)?;
    let tasks = api.fetch_worklogs(range.clone())?;

    let actually_works = api.actually_works()?;
    let output = format!(
        "{}{}",
        Calendar::render(range, tasks, config.show_weekends)?,
        Calendar::works_on(actually_works)
    );
    println!("{}", output);
    cache.save(&output);

    Ok(())
}

// let tasks = vec![
//     WorkLog {
//         day: Utc.ymd(2025, 3, 3).and_hms(8, 0, 0),
//         task: "DD-2165".to_string(),
//         time_spent: "1d".to_string(),
//     },
//     WorkLog {
//         day: Utc.ymd(2025, 3, 4).and_hms(8, 0, 0),
//         task: "DD-2165".to_string(),
//         time_spent: "1d".to_string(),
//     },
//     WorkLog {
//         day: Utc.ymd(2025, 2, 24).and_hms(8, 0, 0),
//         task: "PRIC-1954".to_string(),
//         time_spent: "1d".to_string(),
//     },
//     WorkLog {
//         day: Utc.ymd(2025, 2, 25).and_hms(8, 0, 0),
//         task: "PRIC-1954".to_string(),
//         time_spent: "1d".to_string(),
//     },
//     WorkLog {
//         day: Utc.ymd(2025, 2, 26).and_hms(8, 0, 0),
//         task: "PRIC-1954".to_string(),
//         time_spent: "1d".to_string(),
//     },
//     WorkLog {
//         day: Utc.ymd(2025, 2, 27).and_hms(8, 7, 0),
//         task: "PRIC-1954".to_string(),
//         time_spent: "1d".to_string(),
//     },
//     WorkLog {
//         day: Utc.ymd(2025, 2, 28).and_hms(9, 0, 0),
//         task: "PRIC-1954".to_string(),
//         time_spent: "1d".to_string(),
//     },
// ];
