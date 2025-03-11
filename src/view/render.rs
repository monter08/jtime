use super::Calendar;
use crate::models::{DateRange, Task, WorkLogList, WorkLogListExt};
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use cli_table::{format::Justify, Cell, CellStruct, Style, Table};
use colored::Colorize;

pub trait Render {
    fn render(range: DateRange, tasks: WorkLogList, show_weekends: bool) -> Result<String>;
    fn works_on(tasks: Vec<Task>) -> String;
}

const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

impl Render for Calendar {
    fn render(range: DateRange, tasks: WorkLogList, show_weekends: bool) -> Result<String> {
        let weekday_limit = if show_weekends { WEEKDAYS.len() } else { 5 };

        let table = range
            .days(show_weekends)
            .chunks(weekday_limit)
            .map(|week| {
                week.iter()
                    .map(|day| render_cell(*day, &tasks.get_by_day(*day)))
                    .collect()
            })
            .collect::<Vec<Vec<CellStruct>>>()
            .table()
            .title(
                WEEKDAYS[0..weekday_limit]
                    .iter()
                    .map(|&day| day.cell().justify(Justify::Center))
                    .collect::<Vec<_>>(),
            )
            .bold(true);

        Ok(table.display()?.to_string())
    }

    fn works_on(tasks: Vec<Task>) -> String {
        format!(
            "Actually you work on:\n{}",
            tasks
                .iter()
                .map(|f| format!("[{}] {}", f.id.green(), f.name))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn render_cell(day: NaiveDate, tasks: &WorkLogList) -> CellStruct {
    // Style day number based on conditions
    let day_num = {
        let num = day.day().to_string().yellow();

        let is_weekend = day.weekday().number_from_monday() >= 6;
        let is_empty_today = tasks.is_empty() && day == chrono::Local::now().naive_local().date();
        let is_today = day == chrono::Local::now().naive_local().date();

        match (is_weekend, is_empty_today, is_today) {
            (true, _, _) => num.dimmed(),
            (_, true, _) => num.red(),
            (_, _, true) => num.blue(),
            _ => num,
        }
    };

    // Format task text
    let task_text = tasks
        .iter()
        .map(|t| match t.time_spent.as_str() {
            "1d" => t.task.green().to_string(),
            _ => format!("{} ({})", t.task.green(), t.time_spent.dimmed()),
        })
        .collect::<Vec<_>>()
        .join("\n");

    let task_text = if task_text.is_empty() {
        "-".to_string()
    } else {
        task_text
    };

    let ansi_regex = regex::Regex::new(r"\x1B\[[0-9;]*[mK]").unwrap();
    let width = task_text
        .lines()
        .map(|line| ansi_regex.replace_all(line, "").len())
        .max()
        .unwrap_or(0);

    format!("{:^width$}\n{}", day_num, task_text, width = width)
        .cell()
        .justify(Justify::Center)
}
