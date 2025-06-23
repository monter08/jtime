use crate::{
    api::Jira,
    cache::Cache,
    config::Config,
    view::{helper::Helper, Calendar, Render},
};
use anyhow::Result;

use crate::api::Nager;
use chrono::{Datelike, Utc};

pub fn execute(
    config: &Config,
    api: &Jira,
    nager: &Nager,
    use_cache: &bool,
    month: &Option<u32>,
) -> Result<()> {
    let cache = Cache::new("month".to_string());

    if *use_cache {
        if let Some(data) = cache.load()? {
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
        Calendar::render(
            range,
            tasks,
            config.show_weekends,
            Some(nager.get_all_holidays_map(Utc::now().year().to_string())?)
        )?,
        Calendar::works_on(actually_works)
    );
    println!("{}", output);
    cache.save(&output)?;

    Ok(())
}
