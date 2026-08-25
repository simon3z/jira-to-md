mod cli;
mod config;
mod jira;
mod models;
mod transformer;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use jira::JiraClient;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Init = cli.command {
        let path =
            Config::create_default_config_dir().context("Failed to create default config")?;
        let default_content = r#"[jira]
    url = "https://your-domain.atlassian.net"
    user = "your-email@example.com"
    token = "your-api-token"
    "#;
        fs::write(&path, default_content).context("Failed to write default config file")?;
        println!("Default configuration created at: {:?}", path);
        return Ok(());
    }

    let config = match cli.config {
        Some(custom_config_path) => {
            let content = fs::read_to_string(custom_config_path)
                .context("Failed to read custom config file")?;
            toml::from_str(&content).context("Failed to parse custom config file")?
        }
        None => {
            Config::load().context("Failed to load configuration. Use --config to specify a file or ensure $XDG_CONFIG_HOME/jira-to-md/config.toml exists.")?
        }
    };

    let client = JiraClient::new(config.jira.clone());

    match cli.command {
        Commands::Fetch { key } => {
            let issue_result = client.fetch_issue(&key).await;
            let output_path = cli.output.as_deref();
            match issue_result {
                Ok(issue) => {
                    if cli.json {
                        print_or_write(serde_json::to_string_pretty(&issue).unwrap(), output_path);
                    } else {
                        let markdown = transformer::issue_to_markdown(&issue);
                        print_or_write(markdown, output_path);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Search { jql } => {
            let search_result = client.search_issues(&jql).await;
            let output_path = cli.output.as_deref();
            match search_result {
                Ok(issues) => {
                    if issues.is_empty() {
                        println!("No issues found for JQL: {}", jql);
                    } else if cli.json {
                        print_or_write(serde_json::to_string_pretty(&issues).unwrap(), output_path);
                    } else {
                        let mut combined_markdown = String::new();
                        for issue in issues {
                            combined_markdown.push_str(&transformer::issue_to_markdown(&issue));
                            combined_markdown.push_str("\n---\n\n");
                        }
                        print_or_write(combined_markdown, output_path);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Init => unreachable!(),
    }

    Ok(())
}

fn print_or_write(content: String, output_path: Option<&str>) {
    if let Some(path) = output_path {
        if let Err(e) = fs::write(path, content) {
            eprintln!("Failed to write output file: {}", e);
        }
    } else {
        println!("{}", content);
    }
}
