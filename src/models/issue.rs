use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraIssue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    pub summary: String,
    pub description: Option<serde_json::Value>,
    pub status: Status,
    pub assignee: Option<User>,
    pub priority: Option<Priority>,
    pub created: Option<String>,
    pub updated: Option<String>,
    #[serde(rename = "resolutiondate")]
    pub resolution_date: Option<String>,
    pub labels: Option<Vec<String>>,
    #[serde(rename = "issuelinks")]
    pub issuelinks: Option<Vec<IssueLink>>,
    #[serde(rename = "fixVersions")]
    pub fix_versions: Option<Vec<Version>>,
    pub versions: Option<Vec<Version>>,
    pub reporter: Option<User>,
    pub creator: Option<User>,
    pub project: Option<Project>,
    #[serde(rename = "comment")]
    pub comments: Option<Comments>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLink {
    pub id: String,
    pub inward: Option<IssueLinkRelation>,
    pub outward: Option<IssueLinkRelation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLinkRelation {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub body: serde_json::Value,
}
