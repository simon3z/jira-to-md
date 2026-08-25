# Jira to Markdown Tools

[![github][badge-github]][link-github]
[![license][badge-license]][link-license]
[![version][badge-version]][link-crate]
[![rust-edition][badge-rust]][link-rust]
[![dependencies][badge-deps]][link-cargo]
[![tests][badge-tests]][link-ci]

[badge-github]: https://img.shields.io/badge/github-simon3z/jira--to--md-6f57b0.svg?logo=github
[badge-license]: https://img.shields.io/badge/license-Apache_2.0-blue.svg
[badge-version]: https://img.shields.io/badge/version-0.1.0-ff8000.svg
[badge-rust]: https://img.shields.io/badge/rust-edition_2024-steelblue.svg
[badge-deps]: https://img.shields.io/badge/dependencies-9-green.svg
[badge-tests]: https://img.shields.io/badge/tests-29_passing-brightgreen.svg
[link-github]: https://github.com/simon3z/jira-to-md
[link-license]: LICENSE
[link-crate]: https://crates.io
[link-cargo]: Cargo.toml
[link-rust]: https://www.rust-lang.org
[link-ci]: #ci

A lightweight Rust CLI that transforms Jira issues into clean, LLM-ready Markdown.

## Features

- **Fetch single issues:** Retrieve detailed markdown for a specific Jira key.
- **Search with JQL:** Use Jira Query Language to find multiple issues and aggregate them into a single markdown document.
- **LLM-Optimized:** Automatically parses Atlassian Document Format (ADF) to convert complex descriptions, comments, and lists into clean Markdown.
- **Configurable:** Uses a standard XDG configuration file for easy setup.
- **JSON Output:** Supports raw JSON (`--json`) output for machine-readable consumption.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd jira-to-md
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

The binary will be located at `target/release/jira-to-md`.

## Configuration

The tool looks for a configuration file at `$XDG_CONFIG_HOME/jira-to-md/config.toml`.

To generate a default configuration template, run:
```bash
jira-to-md init
```

### Configuration Example

```toml
[jira]
url = "https://your-domain.atlassian.net"
user = "your-email@example.com"
token = "your-api-token"
```

## Usage

### Fetch a single issue
```bash
jira-to-md fetch PROJ-123
```

### Search for issues using JQL
```bash
jira-to-md search "project = PROJ AND status = 'In Progress'"
```

### Save output to a file
Use the `--output` (or `-o`) flag to save the markdown content to a file instead of printing to stdout.
```bash
jira-to-md search "project = PROJ" --output issues.md
```

### JSON Output
Use the `--json` flag to output raw JSON instead of Markdown.
```bash
jira-to-md --json fetch PROJ-123
```

### Overriding Configuration
Use the `--config` flag to specify a custom configuration file location.
```bash
jira-to-md --config /path/to/custom_config.toml fetch PROJ-123
```

## Agent Skills

Jira to Markdown Tools provides pre-defined skills in the `skills/` directory that can be used by AI agents to interact with Jira more effectively.

- `jira-issue-fetcher`: Fetches details or searches for Jira issues using the Jira to Markdown Tools CLI.

### Using Skills
Agents can load these skills to automate tasks such as retrieving issue information or performing complex JQL searches.

## Pi agent extension

`pi` (the pi coding agent) can drive Jira to Markdown Tools through a bundled extension in
[`pi-extensions/`](pi-extensions/) that exposes `fetch_issue` and `search_issues` tools to the agent.
The extension shells out to the compiled CLI, so install the CLI first (see [Installation](#installation)
and [Configuration](#configuration)). Then install the repo as a pi package
(`pi install git:<host>/user/jira-to-md@<ref>`), and reload pi (`/reload`) to activate.
See [`pi-extensions/README.md`](pi-extensions/README.md) for the full install, verification,
and troubleshooting steps.

## Architecture

Jira to Markdown Tools follows a simple pipeline: **Input (CLI/Config) → Fetch (Jira API) → Transform (ADF → Markdown) → Output (stdout/file)**.

### Data flow

1. `main.rs` loads credentials from the config file.
2. `JiraClient` (`src/jira/client.rs`) makes an authenticated GET request to the Jira REST API v3.
3. The response is deserialized into `JiraIssue` structs (`src/models/`). Because Jira uses Atlassian Document Format (ADF), description/comment fields are held as `serde_json::Value` for flexible parsing.
4. `transformer/adf.rs` recursively converts ADF nodes into Markdown; `transformer/markdown.rs` assembles the full document.

### Key decisions

- **Async** (`tokio` + `reqwest`) for non-blocking network calls.
- **ADF-aware transformation** to preserve lists, links, and other rich content.
- **XDG-compliant** config placement under `~/.config/jira-to-md/`.

## Contributing

Thanks for improving Jira to Markdown Tools! Ways to contribute:

- **Report bugs** or **suggest enhancements** by opening an issue on the GitHub repository. Discuss new features before implementing them.
- **Open a pull request** with a clear description of your changes. Make sure `cargo build`, `cargo fmt`, and `cargo clippy` pass first.

### Coding Standards

- Idiomatic Rust; error handling via `anyhow::Result`; asynchronous operations with `tokio`.
- Run `cargo fmt` and `cargo clippy` before committing.
- Document public items with doc comments (`///`).

### Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification to keep a clean, readable history (e.g., `feat:`, `fix:`, `docs:`, `chore:`).

## License

This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
