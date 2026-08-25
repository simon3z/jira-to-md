# jira-to-md CLI Usage Reference

This document provides detailed usage patterns for the `jira-to-md` CLI tool.

## Command Overview

The `jira-to-md` tool fetches Jira issues via the Atlassian REST API and converts
them into clean, structured Markdown optimized for LLM consumption. It handles
Atlassian Document Format (ADF) parsing for complex descriptions and comments.

### Basic Syntax

```bash
jira-to-md [SUBCOMMAND] [OPTIONS]
```

## Available Options

| Flag | Long Flag | Description |
| :--- | :--- | :--- |
| `-o` | `--output <PATH>` | Save output to the specified file instead of stdout |
| `-j` | `--json` | Print raw JSON API response instead of Markdown |
| `-c` | `--config <PATH>` | Path to a custom configuration file |
| `-h` | `--help` | Print help message |
| `-V` | `--version` | Print version |

## Subcommands

### fetch

Fetch a single issue by its Jira key.

```bash
jira-to-md fetch <ISSUE_KEY>
```

- **Required**: A valid Jira issue key (e.g., `PROJ-123`)
- **Output**: Structured Markdown with metadata, description, and comments
- **Use case**: Getting details on a specific ticket

### search

Search for issues using JQL (Jira Query Language).

```bash
jira-to-md search "<JQL_QUERY>"
```

- **Required**: A JQL query string in double quotes
- **Output**: Concatenated Markdown for all matching issues, separated by `---`
- **Pagination**: Automatically fetches all pages (100 issues per page)
- **Use case**: Bulk retrieval of issues matching criteria

### init

Initialize a default configuration file.

```bash
jira-to-md init
```

- Creates a config template at `$XDG_CONFIG_HOME/jira-to-md/config.toml`
- Prints `Default configuration created at: <path>` on success
- Run this first if you don't have a config file yet

## Detailed Examples

### 1. Fetch a Single Issue

**Standard fetch:**
```bash
jira-to-md fetch PROJ-123
```

**Save to file:**
```bash
jira-to-md fetch PROJ-123 --output issue.md
```

**JSON output:**
```bash
jira-to-md --json fetch PROJ-123
```

### 2. Search for Issues Using JQL

**Standard search:**
```bash
jira-to-md search "project = PROJ AND status = 'In Progress'"
```

**Search with output file:**
```bash
jira-to-md search "project = PROJ" --output issues.md
```

**Complex JQL query:**
```bash
jira-to-md search "project = PROJ AND type in (Bug, Story) AND status != Done ORDER BY priority DESC"
```

**Search with pagination (large result sets):**
```bash
jira-to-md search "project = PROJ AND assignee = currentUser()" --output my-tasks.md
```

### 3. Overriding Configuration

```bash
jira-to-md -c /path/to/custom_config.toml fetch PROJ-123
```

## Configuration Reference

The tool requires a TOML configuration file at `$XDG_CONFIG_HOME/jira-to-md/config.toml`.

### Structure

```toml
[jira]
url = "https://your-domain.atlassian.net"
user = "your-email@example.com"
token = "your-api-token"
```

| Field | Description | Example |
| :--- | :--- | :--- |
| `url` | Your Jira instance base URL (no trailing slash required) | `https://example.atlassian.net` |
| `user` | Email address associated with your Atlassian account | `you@company.com` |
| `token` | REST API token (NOT your password) | `ABCDEF...` |

**Getting an API token:**
1. Go to https://id.atlassian.com/manage-profile/security/api-tokens
2. Click "Create API token"
3. Copy the token value

### Custom Config Path

Use `--config` or `-c` to specify an alternative config file:

```bash
jira-to-md -c ./temp-config.toml fetch PROJ-123
```

## Output Format

Each issue is rendered as structured Markdown:

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

### Metadata fields

| Field | Source | Notes |
| :--- | :--- | :--- |
| Status | `fields.status.name` | Always included |
| Priority | `fields.priority.name` | Only if set |
| Assignee | `fields.assignee.displayName` | Only if assigned |
| Reporter | `fields.reporter.displayName` | Present and not identical to the creator (see dedup below) |
| Creator | `fields.creator.displayName` | Shown unless a same-named reporter already emitted it; also shown when only a creator exists |
| Created | `fields.created` (date portion) | ISO date extracted from timestamp |
| Updated | `fields.updated` (date portion) | ISO date extracted from timestamp |
| Resolved | `fields.resolutiondate` (date portion) | Only if resolved |
| Fix Versions | `fields.fixVersions[].name` | Comma-separated list |
| Versions | `fields.versions[].name` | Comma-separated list |
| Project | `fields.project.name` | Always included |
| Labels | `fields.labels` | Parsed but not displayed in Markdown |

**Reporter/Creator deduplication:** If the reporter and creator are the same person, only `Creator` is shown. Otherwise both `Reporter` and `Creator` appear (each only when that role is populated).

### ADF conversion

The tool recursively parses Atlassian Document Format (ADF) JSON into Markdown:

- **doc / paragraph** → standard text blocks
- **heading** → `#`, `##`, `###` etc.
- **bulletList** → `- item` bullet lists
- **orderedList** → `1. item` numbered lists
- **text with marks** → `**bold**`, `*italic*`, `[link](url)`
- **mention** → rendered as plain text
- **inlineCard** → `[card](url)` links

## Error Handling

On any fetch/search failure the tool prints `Error: <message>` to stderr and exits with code 1. The table lists the real fragments emitted (with their nested cause).

| Error fragment | Cause | Fix |
| :--- | :--- | :--- |
| `Failed to load configuration. Use --config...` (cause: `Configuration file not found`) | No config at the expected XDG path | Run `jira-to-md init`, then fill in `url`, `user`, and `token` in `$XDG_CONFIG_HOME/jira-to-md/config.toml` |
| `Failed to parse configuration file` / `Failed to parse custom config file` | Invalid TOML syntax | Check the config file for syntax errors |
| `Jira API error: 401 - ...` | Invalid email or API token | Verify credentials at https://id.atlassian.com/manage-profile/security/api-tokens |
| `Jira API error: 404 - ...` | Issue key doesn't exist, or wrong instance | Verify the key and Jira URL |
| `Failed to send request to <url>` | Network or DNS issue | Check internet connection |
| `No issues found for JQL: <query>` | Query matched nothing (not an error) | Narrow or adjust your JQL query |

To inspect raw API content when something looks wrong, re-run with `--json` and compare the response body.

## Advanced Usage

### JQL operator cheat sheet

| Operator | Example | Description |
| :--- | :--- | :--- |
| `=` | `status = 'In Progress'` | Equality match (strings need quotes) |
| `!=` | `type != 'Bug'` | Inequality |
| `in` | `type in (Bug, Story)` | Membership |
| `not in` | `status not in (Done, Closed)` | Negated membership |
| `is` | `assignee is currentUser()` | Special functions |
| `>` / `<` | `created > -1w` | Date comparisons |
| `AND` / `OR` | `project = PROJ AND status = Open` | Boolean logic |

### Pagination behavior

The search command paginates automatically:
- Fetches 100 issues per page (`maxResults=100`)
- Increments `startAt` until no more results
- All results are concatenated into a single output

For very large result sets, consider narrowing your JQL query.
