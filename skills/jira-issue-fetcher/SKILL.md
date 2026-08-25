---
name: jira-issue-fetcher
description: >
  Fetch Jira issues and convert them to clean, LLM-optimized Markdown.
  Use when the user asks to fetch, look up, retrieve, search, or analyze
  a Jira issue, ticket, or bug — even without using the word "Jira".
  Includes: single-issue fetch by key (e.g. PROJ-123), JQL-based
  multi-issue search, raw JSON output, and output to file. Trigger
  keywords: Jira, ticket, bug, issue, PROJ-XXX, JQL, backlog, sprint,
  status, assignee, epic, story, task, subtask, atlassian.
license: MIT
metadata:
  version: "1.0"
---

## Overview

Use `jira-to-md` to fetch Jira issues and output structured Markdown optimized
for LLM consumption. The tool parses Atlassian Document Format (ADF) into clean
Markdown, including descriptions, comments, lists, headings, and inline formatting.

## Prerequisites

Before fetching issues, ensure configuration exists:

- Config lives at `$XDG_CONFIG_HOME/jira-to-md/config.toml` (typically `~/.config/jira-to-md/config.toml`)
- If config is missing, run `jira-to-md init` first to create a template
- The config requires `url`, `user` (email), and `token` (API token)

## Commands

### Fetch a single issue

```bash
jira-to-md fetch <ISSUE_KEY>
```

Example: `jira-to-md fetch PROJ-123`

This returns the full issue: summary, status, assignee, priority, description
(converted from ADF), and all comments.

### Search with JQL

```bash
jira-to-md search "<JQL_QUERY>"
```

Example: `jira-to-md search "project = PROJ AND status = 'In Progress' ORDER BY priority DESC"`

Returns all matching issues concatenated with `---` separators. Supports the
full JQL syntax. The tool paginates automatically (100 results per page).

### Output to file

Use `-o` / `--output` to save instead of printing to stdout:

```bash
jira-to-md fetch PROJ-123 --output issue.md
jira-to-md search "project = PROJ" --output all-issues.md
```

### JSON output

Use `--json` (or `-j`) to see the raw API response as JSON instead of Markdown:

```bash
jira-to-md --json fetch PROJ-123
```

### Custom config path

Use `-c` / `--config` to override the config file location:

```bash
jira-to-md -c /path/to/custom.toml fetch PROJ-123
```

## Output Format

Each issue renders as structured Markdown:

```markdown
# [PROJ-123] Issue Summary
Status: Open
Priority: High
Assignee: John Doe
Reporter: Jane Smith
Created: 2025-01-15
Updated: 2025-03-20

## Description

Description text here...

## Comments

**Comment 1**: First comment text...
**Comment 2**: Second comment text...
```

Fields included: Status, Priority, Assignee, Reporter/Creator, Created, Updated,
Resolved (if applicable), Fix Versions, Versions, Project. Reporter and Creator
are deduplicated — only `Creator` is shown when they match.

ADF content in descriptions and comments is converted to proper Markdown including
headings, bold, italic, links, bullet lists, and numbered lists.

## Common Workflows

### Fetch and analyze a single issue

```bash
jira-to-md fetch PROJ-123
```

### Get all open bugs for a project

```bash
jira-to-md search "project = PROJ AND type = Bug AND status != Done"
```

### Find issues assigned to a specific person

```bash
jira-to-md search "assignee = 'John Doe' AND status != Done"
```

### Get sprint progress for a team

```bash
jira-to-md search "project = PROJ AND sprint = 'Sprint 5' AND status != Done"
```

### Save search results to a file for later review

```bash
jira-to-md search "project = PROJ AND status = 'In Progress'" --output in-progress.md
```

## Gotchas

- **Issue keys are case-sensitive**: `proj-123` will fail; use `PROJ-123`.
- **JQL with special characters**: Quote the entire query string. Spaces, quotes,
  and operators like `=` must be inside the JQL query string, not passed as CLI args.
  Example: `jira-to-md search "status = 'In Progress'"` — not `jira-to-md search status = 'In Progress'`.
- **Search pagination**: The tool fetches all pages automatically, but very large
  result sets may take time. The API returns 100 issues per page.
- **Missing fields**: Assignee, priority, and resolution_date are optional in the
  API response and won't appear in output if not set on the issue.
- **ADF parsing**: Complex ADF structures (tables, code blocks, macros) may not
  convert perfectly. Use `--json` to inspect raw content if output looks wrong.
- **Authentication**: The tool uses HTTP Basic Auth with email + API token. If you
  get 401, verify the credentials in config.toml. The token is NOT your password —
  it's a REST API token from your Atlassian account settings.

## Reference

For detailed command syntax, usage examples, and error handling, read
`references/usage.md`.
