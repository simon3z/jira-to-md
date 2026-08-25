use serde_json::{Map, Value};

/// Convert an Atlassian Document Format (ADF) node tree into Markdown.
pub fn adf_to_markdown(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Object(map) => dispatch(map),
        _ => String::new(),
    }
}

fn dispatch(map: &Map<String, Value>) -> String {
    let Some(node_type) = map.get("type").and_then(|v| v.as_str()) else {
        return String::new();
    };
    match node_type {
        "doc" | "listItem" => render_children(map),
        "paragraph" => {
            let children = render_children(map);
            if children.trim().is_empty() {
                children
            } else {
                format!("{children}\n")
            }
        }
        "heading" => render_heading(map),
        "bulletList" => render_bullets(map),
        "orderedList" => render_ordered_list(map),
        "text" => render_text(map),
        "mention" => render_mention(map),
        "inlineCard" => render_inline_card(map),
        _ => String::new(),
    }
}

/// Recursively render a node's `content` array.
fn render_children(map: &Map<String, Value>) -> String {
    let mut md = String::new();
    if let Some(content) = map.get("content").and_then(|v| v.as_array()) {
        for item in content {
            md.push_str(&adf_to_markdown(item));
        }
    }
    md
}

fn render_heading(map: &Map<String, Value>) -> String {
    let level = map
        .get("attrs")
        .and_then(|a| a.get("level"))
        .and_then(|l| l.as_u64())
        .unwrap_or(1) as usize;
    let content = render_children(map);
    format!("\n{} {}\n", "#".repeat(level), content.trim())
}

fn render_bullets(map: &Map<String, Value>) -> String {
    let mut md = String::new();
    if let Some(content) = map.get("content").and_then(|v| v.as_array()) {
        for item in content {
            md.push_str(&adf_to_list_item(item, "- "));
        }
    }
    md
}

fn render_ordered_list(map: &Map<String, Value>) -> String {
    let mut md = String::new();
    if let Some(content) = map.get("content").and_then(|v| v.as_array()) {
        for (i, item) in content.iter().enumerate() {
            md.push_str(&adf_to_list_item(item, &format!("{}. ", i + 1)));
        }
    }
    md
}

fn render_text(map: &Map<String, Value>) -> String {
    let mut text = map
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if let Some(marks) = map.get("marks").and_then(|m| m.as_array()) {
        for mark in marks {
            if mark.get("type").and_then(|t| t.as_str()).is_some() {
                text = apply_mark(text, mark);
            }
        }
    }
    text
}

/// Apply a single formatting mark (bold / italic / link) to `text`.
fn apply_mark(text: String, mark: &Value) -> String {
    match mark.get("type").and_then(|t| t.as_str()) {
        Some("strong") => format!("**{text}**"),
        Some("em") => format!("*{text}*"),
        Some("link") => match link_href(mark) {
            Some(href) => format!("[{text}]({href})"),
            None => text,
        },
        _ => text,
    }
}

fn link_href(mark: &Value) -> Option<&str> {
    mark.get("attrs")
        .and_then(|a| a.get("href"))
        .and_then(|h| h.as_str())
}

fn render_mention(map: &Map<String, Value>) -> String {
    map.get("attrs")
        .and_then(|a| a.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string()
}

fn render_inline_card(map: &Map<String, Value>) -> String {
    match map
        .get("attrs")
        .and_then(|a| a.get("url"))
        .and_then(|u| u.as_str())
    {
        Some(url) => format!("[card]({url})"),
        None => String::new(),
    }
}

fn adf_to_list_item(item: &Value, prefix: &str) -> String {
    let mut md = String::new();
    if let Some(content) = item.get("content").and_then(|c| c.as_array()) {
        for child in content {
            md.push_str(&adf_to_markdown(child));
        }
    }
    format!("{prefix}{}\n", md.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn md(value: Value) -> String {
        adf_to_markdown(&value)
    }

    #[test]
    fn renders_plain_text() {
        assert_eq!(md(json!({"type": "text", "text": "Hello"})), "Hello");
    }

    #[test]
    fn renders_empty_text_as_blank() {
        assert_eq!(md(json!({"type": "text", "text": ""})), "");
    }

    #[test]
    fn renders_bold_mark() {
        assert_eq!(
            md(json!({
                "type": "text", "text": "Hi",
                "marks": [{"type": "strong"}]
            })),
            "**Hi**"
        );
    }

    #[test]
    fn renders_italic_mark() {
        assert_eq!(
            md(json!({
                "type": "text", "text": "Hi",
                "marks": [{"type": "em"}]
            })),
            "*Hi*"
        );
    }

    #[test]
    fn renders_link_mark() {
        assert_eq!(
            md(json!({
                "type": "text", "text": "link",
                "marks": [{"type": "link", "attrs": {"href": "https://example.com"}}]
            })),
            "[link](https://example.com)"
        );
    }

    #[test]
    fn renders_heading_level_one() {
        assert_eq!(
            md(json!({
                "type": "heading",
                "attrs": {"level": 1},
                "content": [{"type": "text", "text": "Title"}]
            })),
            "\n# Title\n"
        );
    }

    #[test]
    fn renders_heading_level_three() {
        assert_eq!(
            md(json!({
                "type": "heading",
                "attrs": {"level": 3},
                "content": [{"type": "text", "text": "Title"}]
            })),
            "\n### Title\n"
        );
    }

    #[test]
    fn renders_heading_defaults_to_level_one() {
        assert_eq!(
            md(json!({
                "type": "heading",
                "content": [{"type": "text", "text": "Title"}]
            })),
            "\n# Title\n"
        );
    }

    #[test]
    fn renders_bullet_list() {
        assert_eq!(
            md(json!({
                "type": "bulletList",
                "content": [
                    {"type": "listItem", "content": [{"type": "text", "text": "A"}]},
                    {"type": "listItem", "content": [{"type": "text", "text": "B"}]}
                ]
            })),
            "- A\n- B\n"
        );
    }

    #[test]
    fn renders_ordered_list() {
        assert_eq!(
            md(json!({
                "type": "orderedList",
                "content": [
                    {"type": "listItem", "content": [{"type": "text", "text": "A"}]},
                    {"type": "listItem", "content": [{"type": "text", "text": "B"}]}
                ]
            })),
            "1. A\n2. B\n"
        );
    }

    #[test]
    fn renders_mention() {
        assert_eq!(
            md(json!({"type": "mention", "attrs": {"text": "@user"}})),
            "@user"
        );
    }

    #[test]
    fn renders_inline_card() {
        assert_eq!(
            md(json!({"type": "inlineCard", "attrs": {"url": "https://x.com"}})),
            "[card](https://x.com)"
        );
    }

    #[test]
    fn unknown_node_type_is_empty() {
        assert_eq!(md(json!({"type": "weird"})), "");
    }

    #[test]
    fn nested_heading_with_bold_text() {
        assert_eq!(
            md(json!({
                "type": "heading",
                "attrs": {"level": 2},
                "content": [{"type": "paragraph", "content": [
                    {"type": "text", "text": "Bold", "marks": [{"type": "strong"}]}
                ]}]
            })),
            "\n## **Bold**\n"
        );
    }
}
