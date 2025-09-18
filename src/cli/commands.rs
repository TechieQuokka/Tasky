use colored::*;
use prettytable::{Cell, Row, Table};

use crate::cli::args::Commands;
use crate::error::Result;
use crate::models::{CreateTodo, Priority, SortBy, SortOrder, Status, Todo, TodoFilter, UpdateTodo};
use crate::services::{TodoService, DefaultTodoService};
use crate::utils;

pub fn execute(command: Commands) -> Result<()> {
  let service = DefaultTodoService::default()?;

  // match command {
  //   Commands::Add { title, description, priority, due } => {
  //     handle_add
  //   }
  // }
}

fn handle_add(
  service: &impl TodoService,
  title: String,
  description: Option<String>,
  priority: String,
  due: Option<String>,
) -> Result<()> {

  let mut create_todo = CreateTodo::new(title);

  if let Some(desc) = description {
    create_todo = create_todo.with_description(desc);
  }

  let priority = Priority::from_str(&priority)?;
  create_todo = create_todo.with_priority(priority)
}