use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tasky")]
#[command(author = "Ryou")]
#[command(version = "0.1.0")]
#[command(about = "개인용 할일 관리 CLI 도구", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Add {
    title: String,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long, default_value = "medium")]
    priority: String,
    #[arg(short = 'd', long)]
    due: Option<String>,
  },

  List {
    #[arg(short, long)]
    status: Option<String>,
    #[arg(short, long)]
    priority: Option<String>,
    
  }
}