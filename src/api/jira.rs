use crate::models::{DateRange, Task, WorkLog, WorkLogList};
use anyhow::Result;
use chrono::{NaiveDate, TimeZone, Utc};
use reqwest::blocking::Client;
use serde_json::Value;

pub struct Jira {
    client: Client,
    url: String,
    user_id: Option<String>,
    token: String,
}

impl Jira {
    pub fn new(url: String, token: String) -> Self {
        Jira {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .expect("Failed to build client"),
            url,
            user_id: None,
            token,
        }
    }

    pub fn get_user_id(&self) -> Result<String> {
        if let Some(user_id) = &self.user_id {
            return Ok(user_id.clone());
        }

        let url = format!("{}/rest/api/2/myself", self.url);

        let response = self.client.get(&url).bearer_auth(&self.token).send()?;

        if response.status().is_success() {
            let users: Value = response.json()?;
            Ok(users["key"]
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| anyhow::anyhow!("User key not found in response"))?)
        } else {
            Err(anyhow::anyhow!("Failed to fetch user ID"))
        }
    }

    pub fn log_worktime(&self, task: &str, time_spent: u64, date: &NaiveDate) -> Result<bool> {
        let json_body = serde_json::json!({
            "started": format!("{}T08:00:00.000+0000", date.format("%Y-%m-%d")),
            "timeSpentSeconds": time_spent
        });

        let url = format!("{}/rest/api/2/issue/{}/worklog", self.url, task);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&json_body)
            .send()?;

        if response.status().is_success() {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("Failed to log time: {}", response.text()?))
        }
    }

    pub fn fetch_worklogs(&self, range: DateRange) -> Result<WorkLogList> {
        let user_id = self.get_user_id()?;
        let url = format!(
            "{}/rest/actonic-tb/1.0/api/worklogs/search-issues",
            self.url
        );
        let request_body = serde_json::json!({
            "startDate": range.from.format("%Y/%m/%d").to_string(),
            "endDate":range.to.format("%Y/%m/%d").to_string(),
            "worklogAuthorId": user_id,
        });

        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .json(&request_body)
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch worklogs: {}", response.status());
        }

        let data: Value = response.json()?;
        let mut entires: WorkLogList = Vec::new();
        if let Some(issues) = data.get("issues").and_then(|i| i.as_array()) {
            for issue in issues {
                if let Some(worklogs) = issue
                    .get("worklog")
                    .and_then(|w| w.get("worklogs"))
                    .and_then(|w| w.as_array())
                {
                    for log in worklogs {
                        if let Some(author_key) = log
                            .get("author")
                            .and_then(|a| a.get("key"))
                            .and_then(|k| k.as_str())
                        {
                            if author_key != user_id {
                                continue;
                            }

                            let day = log
                                .get("started")
                                .and_then(|timestamp| timestamp.as_i64())
                                .and_then(|ts| Utc.timestamp_millis_opt(ts).single())
                                .map(|time| time.to_utc());

                            if let Some(day) = day {
                                let time_spent = log
                                    .get("timeSpent")
                                    .and_then(|t| t.as_str())
                                    .unwrap_or("")
                                    .to_string();

                                let task = log
                                    .get("issueKey")
                                    .and_then(|k| k.as_str())
                                    .unwrap_or("")
                                    .to_string();

                                entires.push(WorkLog {
                                    day,
                                    task,
                                    time_spent,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(entires)
    }

    pub fn actually_works(&self) -> Result<Vec<Task>> {
        let url = format!("{}/rest/api/2/search?jql=assignee=currentUser()%20AND%20statusCategory!=Done&maxResults=50", self.url);

        let response = self.client.get(&url).bearer_auth(&self.token).send()?;
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch issues: {}", response.status());
        }

        let json: Value = response.json().expect("Failed to parse JSON");

        let issues = json
            .get("issues")
            .ok_or_else(|| anyhow::anyhow!("No issues field in response"))?
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Issues is not an array"))?;
        issues
            .iter()
            .map(|issue| -> Result<Task, anyhow::Error> {
                let id = issue
                    .get("key")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Issue key not found"))?;
                let name = issue
                    .get("fields")
                    .and_then(|f| f.get("summary"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Summary not found"))?;
                Ok(Task { id, name })
            })
            .collect()
    }
}
