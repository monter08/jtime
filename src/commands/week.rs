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
    prev: &bool,
    use_cache: &bool,
) -> Result<()> {
    let cache = Cache::new(if *prev { "previous" } else { "current" }.to_string());

    if *use_cache {
        if let Some(data) = cache.load()? {
            println!("{}", data);
            return Ok(());
        }
    }

    let today = Utc::now().date_naive();
    let date = if *prev {
        today - chrono::Duration::days(7)
    } else {
        today
    };

    let range = Calendar::range_days_for_week(date)?;
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
