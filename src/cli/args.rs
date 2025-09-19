use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tasky")]
#[command(author = "Ryou")]
#[command(version = "0.1.8")]
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
    #[arg(long, allow_hyphen_values = true)]
    due: Option<String>,
  },

  List {
    #[arg(short, long)]
    status: Option<String>,
    #[arg(short, long)]
    priority: Option<String>,
    #[arg(long, default_value = "created")]
    sort: String,
    #[arg(long, default_value = "desc")]
    order: String,
    #[arg(long)]
    today: bool,
    #[arg(long)]
    overdue: bool,
    #[arg(long)]
    urgent: bool,
    #[arg(short, long)]
    verbose: bool,
  },

  Show {
    id: i64
  },

  Done {
    ids: Vec<i64>
  },

  Undone {
    id: i64
  },

  Remove {
    id: i64
  },

  Edit {

    id: i64,
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    priority: Option<String>,
    #[arg(long, allow_hyphen_values = true)]
    due: Option<String>,
  },

  Stats,
  Init {
    #[arg(long)]
    force: bool,
  },
  DbInfo,
}