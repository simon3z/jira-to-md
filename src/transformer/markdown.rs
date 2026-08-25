use crate::models::JiraIssue;
use crate::transformer::adf::adf_to_markdown;

/// Render a single Jira issue as structured Markdown.
pub fn issue_to_markdown(issue: &JiraIssue) -> String {
    let mut md = String::new();
    render_header(&mut md, issue);
    render_meta(&mut md, issue);
    render_extra_fields(&mut md, issue);
    render_description(&mut md, issue);
    render_comments(&mut md, issue);
    collapse_blank_lines(md)
}

fn render_header(md: &mut String, issue: &JiraIssue) {
    md.push_str(&format!("# [{}] {}\n", issue.key, issue.fields.summary));
}

fn render_meta(md: &mut String, issue: &JiraIssue) {
    let mut meta = Vec::new();
    meta.push(format!("Status: {}", issue.fields.status.name));
    if let Some(p) = &issue.fields.priority {
        meta.push(format!("Priority: {}", p.name));
    }
    if let Some(a) = &issue.fields.assignee {
        meta.push(format!("Assignee: {}", a.display_name));
    }
    meta.extend(creator_reporter_lines(issue));
    if let Some(cr) = &issue.fields.created {
        meta.push(format!("Created: {}", cr.get(0..10).unwrap_or(cr)));
    }
    if let Some(up) = &issue.fields.updated {
        meta.push(format!("Updated: {}", up.get(0..10).unwrap_or(up)));
    }
    if let Some(res) = &issue.fields.resolution_date {
        meta.push(format!("Resolved: {}", res.get(0..10).unwrap_or(res)));
    }
    md.push_str(&format!("{}\n", meta.join("\n")));
}

/// Emit the Creator/Reporter lines, deduplicating when they refer to the same user.
fn creator_reporter_lines(issue: &JiraIssue) -> Vec<String> {
    let reporter = issue
        .fields
        .reporter
        .as_ref()
        .map(|r| r.display_name.clone());
    let creator = issue
        .fields
        .creator
        .as_ref()
        .map(|c| c.display_name.clone());
    match (reporter.as_ref(), creator.as_ref()) {
        (Some(a), Some(b)) if a == b => vec![format!("Creator: {a}")],
        (Some(r), _) => {
            let mut lines = vec![format!("Reporter: {r}")];
            if let Some(c) = creator {
                lines.push(format!("Creator: {c}"));
            }
            lines
        }
        (_, Some(c)) => vec![format!("Creator: {c}")],
        _ => Vec::new(),
    }
}

fn render_extra_fields(md: &mut String, issue: &JiraIssue) {
    if let Some(fix_versions) = &issue.fields.fix_versions
        && !fix_versions.is_empty()
    {
        md.push_str(&format!(
            "Fix Versions: {}\n",
            fix_versions
                .iter()
                .map(|v| v.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(versions) = &issue.fields.versions
        && !versions.is_empty()
    {
        md.push_str(&format!(
            "Versions: {}\n",
            versions
                .iter()
                .map(|v| v.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(project) = &issue.fields.project {
        md.push_str(&format!("Project: {}\n", project.name));
    }
    md.push('\n');
}

fn render_description(md: &mut String, issue: &JiraIssue) {
    let Some(description) = &issue.fields.description else {
        return;
    };
    let markdown = adf_to_markdown(description);
    let trimmed = markdown.trim();
    if trimmed.is_empty() {
        return;
    }
    md.push_str("## Description\n");
    for line in trimmed.lines() {
        if !line.trim_end().is_empty() {
            md.push_str(&format!("{}\n", line.trim_end()));
        }
    }
}

fn render_comments(md: &mut String, issue: &JiraIssue) {
    let Some(wrapper) = &issue.fields.comments else {
        return;
    };
    if wrapper.comments.is_empty() {
        return;
    }
    md.push_str("## Comments\n");
    for (i, comment) in wrapper.comments.iter().enumerate() {
        md.push_str(&format!("**Comment {}**: ", i + 1));
        let text = adf_to_markdown(&comment.body);
        for line in text.lines() {
            if !line.trim_end().is_empty() {
                md.push_str(&format!("{}\n", line.trim_end()));
            }
        }
    }
    md.push('\n');
}

/// Trim trailing whitespace from each line and collapse multiple blank lines into one.
fn collapse_blank_lines(result: String) -> String {
    let mut output = String::new();
    let mut blank_count = 0;

    for line in result.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            blank_count += 1;
            if blank_count <= 1 {
                output.push('\n');
            }
        } else {
            blank_count = 0;
            output.push_str(trimmed);
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse_issue(json: serde_json::Value) -> JiraIssue {
        serde_json::from_value(json).expect("valid issue")
    }

    #[test]
    fn minimal_issue_renders_header_and_status() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-9",
            "fields": {"summary": "X", "status": {"name": "Open"}}
        })));
        assert!(out.contains("# [PROJ-9] X"));
        assert!(out.contains("Status: Open"));
    }

    #[test]
    fn renders_full_meta_block() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "Test summary",
                "status": {"name": "To Do"},
                "priority": {"name": "High"},
                "assignee": {"displayName": "Alice"},
                "created": "2024-01-15T10:00:00.000+0000",
                "updated": "2024-01-16T10:00:00.000+0000"
            }
        })));
        assert!(out.contains("# [PROJ-1] Test summary"));
        assert!(out.contains("Status: To Do"));
        assert!(out.contains("Priority: High"));
        assert!(out.contains("Assignee: Alice"));
        assert!(out.contains("Created: 2024-01-15"));
        assert!(out.contains("Updated: 2024-01-16"));
    }

    #[test]
    fn dedupes_creator_and_reporter_when_same() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "reporter": {"displayName": "Alice"},
                "creator": {"displayName": "Alice"}
            }
        })));
        assert!(out.contains("Creator: Alice"));
        assert!(!out.contains("Reporter:"));
    }

    #[test]
    fn renders_different_creator_and_reporter() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "reporter": {"displayName": "Alice"},
                "creator": {"displayName": "Bob"}
            }
        })));
        assert!(out.contains("Reporter: Alice"));
        assert!(out.contains("Creator: Bob"));
    }

    #[test]
    fn only_reporter_produces_reporter_line() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "reporter": {"displayName": "Alice"}
            }
        })));
        assert!(out.contains("Reporter: Alice"));
        assert!(!out.contains("Creator:"));
    }

    #[test]
    fn renders_description_from_adf() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "description": {
                    "type": "doc",
                    "content": [{"type": "paragraph", "content": [
                        {"type": "text", "text": "Description text"}
                    ]}]
                }
            }
        })));
        assert!(out.contains("## Description"));
        assert!(out.contains("Description text"));
    }

    #[test]
    fn renders_comments() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "comment": {"comments": [
                    {"body": {"type": "doc", "content": [
                        {"type": "paragraph", "content": [{"type": "text", "text": "First comment"}]}
                    ]}}
                ]}
            }
        })));
        assert!(out.contains("## Comments"));
        assert!(out.contains("**Comment 1**: First comment"));
    }

    #[test]
    fn renders_extra_fields() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "fixVersions": [{"name": "1.0"}, {"name": "1.1"}],
                "project": {"name": "PROJ"}
            }
        })));
        assert!(out.contains("Fix Versions: 1.0, 1.1"));
        assert!(out.contains("Project: PROJ"));
    }

    #[test]
    fn collapses_multiple_blank_lines() {
        let out = issue_to_markdown(&parse_issue(json!({
            "id": "1",
            "key": "PROJ-1",
            "fields": {
                "summary": "S",
                "status": {"name": "To Do"},
                "description": {
                    "type": "doc",
                    "content": [
                        {"type": "paragraph", "content": [{"type": "text", "text": "Line1"}]},
                        {"type": "paragraph", "content": [{"type": "text", "text": "Line2"}]}
                    ]
                }
            }
        })));
        assert!(out.contains("Line1"));
        assert!(out.contains("Line2"));
        // No three consecutive newlines (blank lines collapsed to a single one).
        assert!(!out.contains("\n\n\n"));
    }
}
