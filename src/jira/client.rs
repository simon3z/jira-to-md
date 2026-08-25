use crate::config::JiraConfig;
use crate::models::JiraIssue;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JiraSearchResponse {
    issues: Vec<JiraIssue>,
}

pub struct JiraClient {
    config: JiraConfig,
    client: Client,
}

impl JiraClient {
    pub fn new(config: JiraConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    pub async fn fetch_issue(&self, issue_key: &str) -> Result<JiraIssue> {
        let url = format!(
            "{}/rest/api/3/issue/{}",
            self.config.url.trim_end_matches('/'),
            issue_key
        );
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.config.user, Some(&self.config.token))
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", url))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Jira API error: {} - {}", status, body));
        }

        let body_text = response
            .text()
            .await
            .context("Failed to read response body")?;
        let issue: JiraIssue = serde_json::from_str(&body_text)
            .with_context(|| format!("Failed to deserialize issue. Body: {}", body_text))?;
        Ok(issue)
    }

    pub async fn search_issues(&self, jql: &str) -> Result<Vec<JiraIssue>> {
        let base_url = format!(
            "{}/rest/api/3/search/jql",
            self.config.url.trim_end_matches('/')
        );
        let mut all_issues = Vec::new();
        let mut start_at = 0;
        let page_size = 100;

        loop {
            let url = format!(
                "{}?jql={}&startAt={}&maxResults={}&fields=summary,description,status,assignee,priority,created,updated,resolutiondate,labels,issuelinks,fixVersions,versions,reporter,creator,project,comment",
                base_url,
                urlencoding::encode(jql),
                start_at,
                page_size
            );

            let response = self
                .client
                .get(&url)
                .basic_auth(&self.config.user, Some(&self.config.token))
                .send()
                .await
                .with_context(|| format!("Failed to send request to {}", url))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(anyhow::anyhow!("Jira API error: {} - {}", status, body));
            }

            let body_text = response
                .text()
                .await
                .context("Failed to read response body")?;
            let search_response: JiraSearchResponse = serde_json::from_str(&body_text)
                .with_context(|| {
                    format!("Failed to deserialize search response. Body: {}", body_text)
                })?;

            let issue_count = search_response.issues.len();

            all_issues.extend(search_response.issues);

            if issue_count == 0 || issue_count < page_size {
                break;
            }

            start_at += issue_count;
        }

        Ok(all_issues)
    }
}
