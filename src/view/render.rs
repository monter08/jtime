use super::Calendar;
use crate::api::nager::HolidayMap;
use crate::models::{DateRange, Task, WorkLogList, WorkLogListExt};
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use cli_table::{format::Justify, Cell, CellStruct, Style, Table};
use colored::Colorize;

pub trait Render {
    fn render(
        range: DateRange,
        tasks: WorkLogList,
        show_weekends: bool,
        holiday_map: Option<HolidayMap>,
    ) -> Result<String>;
    fn works_on(tasks: Vec<Task>) -> String;
}

const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn parse_time_to_hours(time_spent: &str) -> f32 {
    if time_spent.ends_with('d') {
        time_spent
            .trim_end_matches('d')
            .parse::<f32>()
            .unwrap_or(0.0)
            * 8.0
    } else if time_spent.ends_with('h') {
        time_spent
            .trim_end_matches('h')
            .parse::<f32>()
            .unwrap_or(0.0)
    } else if time_spent.ends_with('m') {
        time_spent
            .trim_end_matches('m')
            .parse::<f32>()
            .unwrap_or(0.0)
            / 60.0
    } else {
        0.0
    }
}

fn style_day_number(day: NaiveDate, tasks: &WorkLogList, holiday: &Option<String>) -> String {
    let today = chrono::Local::now().naive_local().date();
    let num = day.day().to_string().yellow();
    let is_weekend = day.weekday().number_from_monday() >= 6;
    let is_empty_today = tasks.is_empty() && day == today;
    let is_today = day == today;

    match (holiday.is_some(), is_weekend, is_empty_today, is_today) {
        (true, ..) => num.cyan(),
        (_, true, ..) => num.dimmed(),
        (_, _, true, ..) => num.red(),
        (_, _, _, true) => num.blue(),
        _ => num,
    }
    .to_string()
}

fn format_tasks(tasks: &WorkLogList, holiday: &Option<String>) -> String {
    let mut task_text = tasks
        .iter()
        .map(|t| {
            let task_display = t.task.green();
            match t.time_spent.as_str() {
                "1d" => task_display.to_string(),
                _ => format!("{} ({})", task_display, t.time_spent.dimmed()),
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(holiday_name) = holiday {
        task_text = format!(
            "{}{}",
            holiday_name.bold().cyan(),
            if task_text.is_empty() {
                String::new()
            } else {
                format!("\n{}", task_text)
            }
        );
    }

    if task_text.is_empty() {
        "-".to_string()
    } else {
        task_text
    }
}

fn format_hours_info(
    day: NaiveDate,
    tasks: &WorkLogList,
    total_hours: f32,
    holiday: &Option<String>,
) -> String {
    let today = chrono::Local::now().naive_local().date();
    let is_past_or_today = day <= today;

    if !tasks.is_empty() {
        let total_text = format!("total: {}h", total_hours).on_bright_black().white();

        if total_hours > 8.0 {
            let overflow = total_hours - 8.0;
            let overflow_text = format!("overtime: +{}h", overflow).on_purple().white();
            format!("---\n{}\n{}", total_text, overflow_text)
        } else if total_hours < 8.0 {
            let time_left = 8.0 - total_hours;
            let time_left_text = format!("remaining: -{}h", time_left).on_red().white();
            format!("---\n{}\n{}", total_text, time_left_text)
        } else {
            format!("---\n{}", total_text)
        }
    } else if is_past_or_today && holiday.is_none() {
        let is_weekend = day.weekday().number_from_monday() >= 6;
        if !is_weekend {
            format!("---\n{}", "remaining: -8h".on_red().white())
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

impl Render for Calendar {
    fn render(
        range: DateRange,
        tasks: WorkLogList,
        show_weekends: bool,
        holiday_map: Option<HolidayMap>,
    ) -> Result<String> {
        let weekday_limit = if show_weekends { WEEKDAYS.len() } else { 5 };
        let holiday_map = holiday_map.unwrap_or_default();
        let table = range
            .days(show_weekends)
            .chunks(weekday_limit)
            .map(|week| {
                week.iter()
                    .map(|day| {
                        render_cell(
                            *day,
                            &tasks.get_by_day(*day),
                            holiday_map
                                .get(&day.format("%Y-%m-%d").to_string())
                                .cloned(),
                        )
                    })
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

fn render_cell(day: NaiveDate, tasks: &WorkLogList, holiday: Option<String>) -> CellStruct {
    let total_hours: f32 = tasks
        .iter()
        .map(|t| parse_time_to_hours(&t.time_spent))
        .sum();

    let day_num = style_day_number(day, tasks, &holiday);
    let task_text = format_tasks(tasks, &holiday);
    let hours_info = format_hours_info(day, tasks, total_hours, &holiday);

    let combined_text = if hours_info.is_empty() {
        task_text
    } else {
        format!("{}\n{}", task_text, hours_info)
    };

    let ansi_regex = regex::Regex::new(r"\x1B\[[0-9;]*[mK]").unwrap();
    let width = combined_text
        .lines()
        .map(|line| ansi_regex.replace_all(line, "").len())
        .max()
        .unwrap_or(0);

    format!("{:^width$}\n{}", day_num, combined_text, width = width)
        .cell()
        .justify(Justify::Center)
}
