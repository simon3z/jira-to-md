# jira-to-md — Pi Agent Extension

A [pi](https://github.com/earendil-works/pi-coding-agent) extension that exposes two
tools to the agent for working with Jira through the `jira-to-md` CLI:

| Tool | Description |
|------|-------------|
| `fetch_issue` | Fetch a single Jira issue by key (e.g. `PROJ-123`) as structured Markdown |
| `search_issues` | Search issues with a JQL query, with automatic pagination |

The agent sees these tools automatically and can call them when you ask about Jira
issues, tickets, bugs, backlogs, sprints, etc.

> **Security:** extensions run with your full system permissions. Only install from
> sources you trust. This extension only runs the local `jira-to-md` binary; it makes
> no network calls of its own.

---

## How it works

```
you  →  pi agent  →  fetch_issue / search_issues tool  →  jira-to-md CLI  →  Markdown back to agent
```

The extension is a thin wrapper. It never talks to Jira directly — it shells out to the
`jira-to-md` binary, so **the CLI must be installed and configured first**. See
[Prerequisites](#prerequisites).

This lives in this repo under [`pi-extensions/`](../pi-extensions/) and is declared as a
pi package by the root [`package.json`](../package.json), so pi discovers it when you
install the repo as a pi package.

---

## Prerequisites

- The `jira-to-md` CLI installed and on your `PATH`. It is **not** bundled with the
  extension, so install it separately (see [Installing the CLI](#installing-the-cli)).
- A Rust toolchain (`cargo`) if you build from source. Otherwise use a prebuilt binary.

## Installing the CLI

### From a local clone (this repo)

```bash
# From inside the jira-to-md directory:
cargo install --path .

# …or, without installing globally, build a release binary you can call by path:
cargo build --release
# binary is at ./target/release/jira-to-md
```

### From source (fresh clone)

```bash
git clone <repo-url> jira-to-md
cd jira-to-md
cargo install --path .
```

### Configure credentials

```bash
jira-to-md init
```

Then edit `$XDG_CONFIG_HOME/jira-to-md/config.toml` (typically
`~/.config/jira-to-md/config.toml`) with your Jira URL, account email, and API token:

```toml
[jira]
url = "https://your-domain.atlassian.net"
user = "your-email@example.com"
token = "your-rest-api-token"
```

Get an API token at <https://id.atlassian.com/manage-profile/security/api-tokens>. It is
**not** your password.

Verify the CLI works before touching the extension:

```bash
jira-to-md fetch PROJ-123
```

---

## Installing the Pi extension

### Option A — as a pi package (recommended)

Install the repo as a pi package. This clones it, pins a ref, and auto-discovers the
extension from the root `package.json`.

```bash
# Global (all projects)
pi install git:<host>/user/jira-to-md@<ref>

# Project-local (this repo only) — also writes .pi/settings.json
pi install -l git:<host>/user/jira-to-md@<ref>
```

Then reload pi so it picks up the extension:

- TUI: run `/reload`
- Or restart pi

In TUI mode you'll see a small `"jira-to-md tools ready"` notification on session start.

---

## Verifying the tools are loaded

Start a session and ask pi to list or use a tool, e.g. *"what can you do with Jira?"* —
the agent should report the `fetch_issue` / `search_issues` tools are available. In TUI
mode, `/reload` re-scans extensions.

---

## Usage

The agent can call the tools on its own when you ask about Jira, e.g.:

- "What's the status of PROJ-123?"
- "Search for all open bugs in PROJ"
- "Show me my assigned tickets"

You can also drive it explicitly:

```bash
# Ask pi to fetch an issue and summarize it
pi "fetch PROJ-123 and tell me what changed"
```

---

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| Tool errors with "jira-to-md failed" / "command not found" | CLI not installed or not on `PATH` | Run `cargo install --path .` (or use the release binary) and confirm `command -v jira-to-md` works. |
| `401 Unauthorized` from Jira | Bad credentials in config | Re-run `jira-to-md init` and check `url`, `user`, `token` in `$XDG_CONFIG_HOME/jira-to-md/config.toml`. |
| Tools don't appear after install | Extension not loaded / project not trusted | `/reload`; for `.pi/extensions`, trust the project first. |
| No issues found | Empty result set, not an error | Expected — try a narrower JQL query. |

---

## Development / quick testing

Test the extension without installing it globally:

```bash
pi -e ./pi-extensions/index.ts
```

This loads `pi-extensions/index.ts` for the current session only. Changes to the file are
picked up on `/reload`.
