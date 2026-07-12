use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod error;
mod generators;

use commands::CommandHandler;
use error::InitiumError;

#[derive(Parser)]
#[command(
    name = "initium",
    about = "Bootstrap project configuration files",
    version,
    long_about = "A CLI tool to quickly generate common project configuration files like .editorconfig, .prettierrc, and more."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Target directory (defaults to current directory)
    #[arg(short, long, value_name = "DIR")]
    target: Option<PathBuf>,

    /// Force overwrite existing files
    #[arg(short, long)]
    force: bool,

    /// Fail if files already exist (default: false - will skip existing files)
    #[arg(short = 'e', long)]
    fail_on_exists: bool,

    /// Show what would be created without actually creating files
    #[arg(long)]
    dry_run: bool,

    /// Generate git hooks for the project
    #[arg(long)]
    hooks: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate configuration files for a basic project
    Basic {
        /// Template to use (e.g., 'google', 'airbnb')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Ruby project
    Ruby {
        /// Template to use (e.g., 'rails', 'sinatra', 'gem')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Python project
    Python {
        /// Template to use (e.g., 'default', 'django', 'flask')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Node.js project
    Node {
        /// Template to use (e.g., 'default', 'express', 'react')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Go project
    Go {
        /// Template to use (e.g., 'default', 'web', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Rust project
    Rust {
        /// Template to use (e.g., 'default', 'web', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Bash project
    Bash {
        /// Template to use (e.g., 'default', 'devops', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Dart project
    Dart {
        /// Template to use (e.g., 'default', 'cli', 'package')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Flutter project
    Flutter {
        /// Template to use (e.g., 'default', 'package', 'plugin')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Automatically detect project type and generate appropriate configs
    Auto,
    /// Interactive mode - guided configuration setup
    Interactive,
    /// List all available configuration files
    List,
}

#[tokio::main]
async fn main() -> Result<(), InitiumError> {
    let cli = Cli::parse();

    let target_dir = cli
        .target
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    if !target_dir.exists() {
        return Err(InitiumError::DirectoryNotFound(target_dir));
    }

    if !target_dir.is_dir() {
        return Err(InitiumError::NotADirectory(target_dir));
    }

    let handler = CommandHandler::new(
        target_dir,
        cli.force,
        cli.fail_on_exists,
        cli.dry_run,
        cli.hooks,
    );

    match cli.command {
        Commands::Basic { template } => handler.handle_basic(template).await?,
        Commands::Ruby { template } => handler.handle_ruby(template).await?,
        Commands::Python { template } => handler.handle_python(template).await?,
        Commands::Node { template } => handler.handle_node(template).await?,
        Commands::Go { template } => handler.handle_go(template).await?,
        Commands::Rust { template } => handler.handle_rust(template).await?,
        Commands::Bash { template } => handler.handle_bash(template).await?,
        Commands::Dart { template } => handler.handle_dart(template).await?,
        Commands::Flutter { template } => handler.handle_flutter(template).await?,
        Commands::Auto => handler.handle_auto().await?,
        Commands::Interactive => handler.handle_interactive().await?,
        Commands::List => handler.handle_list(),
    }

    Ok(())
}
