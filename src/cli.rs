use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output file path. If not provided, prints to stdout.
    #[arg(short, long, global = true)]
    pub output: Option<String>,

    /// Print raw JSON instead of Markdown
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Path to the configuration file
    #[arg(short, long, global = true)]
    pub config: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch a single issue by its key
    Fetch {
        /// The Jira issue key (e.g., PROJ-123)
        key: String,
    },
    /// Search for issues using JQL
    Search {
        /// The JQL query string
        jql: String,
    },
    /// Initialize a default configuration file
    Init,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_fetch_with_json_flag() {
        let cli = Cli::parse_from(["jira-to-md", "--json", "fetch", "PROJ-1"]);
        assert!(cli.json);
        assert!(matches!(cli.command, Commands::Fetch { key } if key == "PROJ-1"));
    }

    #[test]
    fn parses_short_json_flag() {
        let cli = Cli::parse_from(["jira-to-md", "-j", "fetch", "PROJ-2"]);
        assert!(cli.json);
    }

    #[test]
    fn parses_search_command() {
        let cli = Cli::parse_from(["jira-to-md", "search", "project = PROJ"]);
        assert!(matches!(cli.command, Commands::Search { jql } if jql == "project = PROJ"));
    }

    #[test]
    fn parses_output_flag() {
        let cli = Cli::parse_from(["jira-to-md", "-o", "out.md", "fetch", "PROJ-1"]);
        assert_eq!(cli.output.as_deref(), Some("out.md"));
    }

    #[test]
    fn parses_config_flag() {
        let cli = Cli::parse_from(["jira-to-md", "--config", "/tmp/c.toml", "fetch", "PROJ-1"]);
        assert_eq!(
            cli.config.as_deref(),
            Some(std::path::Path::new("/tmp/c.toml"))
        );
    }

    #[test]
    fn default_has_no_json_or_output() {
        let cli = Cli::parse_from(["jira-to-md", "fetch", "PROJ-1"]);
        assert!(!cli.json);
        assert!(cli.output.is_none());
        assert!(cli.config.is_none());
    }
}
