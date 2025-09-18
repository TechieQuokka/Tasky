use colored::*;
use prettytable::{Cell, Row, Table};

use crate::cli::args::Commands;
use crate::error::Result;
use crate::models::{CreateTodo, Priority, SortBy, SortOrder, Status, Todo, TodoFilter, UpdateTodo};
use crate::services::{TodoService, DefaultTodoService};
use crate::utils;

pub fn execute(command: Commands) -> Result<()> {
  let service = DefaultTodoService::default()?;

  match command {
    Commands::Add { title, description, priority, due } => {
      handle_add(&service, title, description, priority, due)
    }
    Commands::List { status, priority, sort, order, today, overdue, urgent } => {
      handle_list(&service, status, priority, sort, order, today, overdue, urgent)
    }
    Commands::Done { id } => {
      handle_done(&service, id)
    }
    Commands::Undone { id } => {
      handle_undone(&service, id)
    }
    Commands::Remove { id } => {
      handle_remove(&service, id)
    }
    Commands::Edit { id, title, description, priority, due } => {
      handle_edit(&service, id, title, description, priority, due)
    }
    Commands::Stats => {
      handle_stats(&service)
    }
    Commands::Init { force } => {
      handle_init(force)
    }
  }
}

fn handle_add(
  service: &impl TodoService,
  title: String,
  description: Option<String>,
  priority: String,
  due: Option<String>,
) -> Result<()> {

  let mut create_todo = CreateTodo::new(title.clone());

  if let Some(desc) = description {
    create_todo = create_todo.with_description(desc);
  }

  let priority = Priority::from_str(&priority)?;
  create_todo = create_todo.with_priority(priority);

  if let Some(due_str) = due {
    let due_date = utils::parse_date(&due_str)?;
    create_todo = create_todo.with_due_date(due_date);
  }

  let todo = service.create_todo(create_todo)?;

  println!("{} 할일이 추가되었습니다!", "✅".green());
  println!("  ID: {}", todo.id.unwrap_or(0).to_string().cyan());
  println!("  제목: {}", todo.title.bold());
  if let Some(desc) = &todo.description {
    println!("  설명: {}", desc);
  }

  println!("  우선순위: {} {}", todo.priority.to_emoji(), todo.priority.to_display_string());
  if let Some(due) = todo.due_date {
      println!("  마감일: {}", utils::format_date(&due).yellow());
  }

  Ok(())
}

fn handle_list(
  service: &impl TodoService,
  status: Option<String>,
  priority: Option<String>,
  sort: String,
  order: String,
  today: bool,
  overdue: bool,
  urgent: bool,
) -> Result<()> {

  let todos = if today {
    service.get_today_todos()?
  } else if overdue {
    service.get_overdue_todos()?
  } else if urgent {
    service.get_urgent_todos()?
  } else {
    let mut filter = TodoFilter::default();

    if let Some(status_str) = status {
      filter.status = Some(Status::from_str(&status_str)?);
    }

    if let Some(priority_str) = priority {
      filter.priority = Some(Priority::from_str(&priority_str)?);
    }

    let sort_by = SortBy::from_str(&sort)?;
    let sort_order = SortOrder::from_str(&order)?;

    service.list_todos(Some(filter), Some(sort_by), Some(sort_order))?
  };

  if todos.is_empty() {
    println!("{}", "할일이 없습니다.".yellow());
    return Ok(());
  }

  print_todos_table(&todos);
  println!("\n총 {}개의 할일", todos.len().to_string().cyan());

  Ok(())
}

fn handle_done (service: &impl TodoService, id: i64) -> Result<()> {
  let todo = service.complete_todo(id)?;
  println!("{} 할일을 완료했습니다!", "✅".green());
  println!("  제목: {}", todo.title.strikethrough());
  Ok(())
}

fn handle_undone(service: &impl TodoService, id: i64) -> Result<()> {
  let todo = service.uncomplete_todo(id)?;
  println!("{} 할일을 다시 대기 상태로 변경했습니다.", "⏳".yellow());
  println!("  제목: {}", todo.title.bold());
  Ok(())
}

fn handle_remove(service: &impl TodoService, id: i64) -> Result<()> {

  let todo = service.get_todo_by_id(id)?;
  let title = todo.title.clone();

  service.delete_todo(id)?;
  println!("{} 할일을 삭제했습니다!", "🗑️".red());
  println!("  제목: {}", title.dimmed());
  Ok(())
}

fn handle_edit(
  service: &impl TodoService,
  id: i64,
  title: Option<String>,
  description: Option<String>,
  priority: Option<String>,
  due: Option<String>,
) -> Result<()> {

  let mut update_todo = UpdateTodo::default();

  if let Some(title) = title {
    update_todo.title = Some(title);
  }
  if let Some(description) = description {
    update_todo.description = Some(description);
  }
  if let Some(priority_str) = priority {
    update_todo.priority = Some(Priority::from_str(&priority_str)?);
  }
  if let Some(due_str) = due {
    update_todo.due_date = Some(utils::parse_date(&due_str)?);
  }

  let todo = service.update_todo(id, update_todo)?;

  println!("{} 할일을 수정했습니다!", "✏️".blue());
  println!("  ID: {}", todo.id.unwrap_or(0).to_string().cyan());
  println!("  제목: {}", todo.title.bold());
  if let Some(desc) = &todo.description {
    println!("  설명: {}", desc);
  }
  println!("  우선순위: {} {}", todo.priority.to_emoji(), todo.priority.to_display_string());
  println!("  상태: {} {}", todo.status.to_emoji(), todo.status.to_display_string());
  if let Some(due) = todo.due_date {
    println!("  마감일: {}", utils::format_date(&due).yellow());
  }

  Ok(())
}

fn handle_stats(service: &impl TodoService) -> Result<()> {
  let stats = service.get_stats()?;

  println!("\n{}", "📊 할일 통계".bold().blue());
  println!("{}", "─".repeat(40));
  
  println!("전체 할일: {}", stats.total_todos.to_string().cyan());
  println!("대기중: {} {}", 
    stats.pending_todos.to_string().yellow(),
    format!("({}%)", ((stats.pending_todos as f64 / stats.total_todos.max(1) as f64) * 100.0) as i32)
  );
  println!("완료: {} {}", 
    stats.completed_todos.to_string().green(),
    format!("({}%)", stats.completion_rate as i32)
  );
  println!("높은 우선순위: {}", stats.high_priority_todos.to_string().red());
  
  if stats.overdue_todos > 0 {
    println!("⚠️  기한 초과: {}", stats.overdue_todos.to_string().red().bold());
  }
  
  println!("{}", "─".repeat(40));
  
  // 진행률 바 표시
  let progress_bar_width = 30;
  let filled = ((stats.completion_rate / 100.0) * progress_bar_width as f64) as usize;
  let empty = progress_bar_width - filled;
  
  print!("완료율: [");
  print!("{}", "█".repeat(filled).green());
  print!("{}", "░".repeat(empty).dimmed());
  println!("] {:.1}%", stats.completion_rate);

  Ok(())
}

fn handle_init(force: bool) -> Result<()> {
  use crate::database::Database;
  
  let db_path = Database::default_path();
  
  if db_path.exists() && !force {
      println!("{} 데이터베이스가 이미 존재합니다.", "⚠️".yellow());
      println!("기존 데이터베이스를 삭제하고 새로 만들려면 --force 옵션을 사용하세요.");
      return Ok(());
  }
  
  if db_path.exists() && force {
      std::fs::remove_file(&db_path)?;
      println!("{} 기존 데이터베이스를 삭제했습니다.", "🗑️".red());
  }
  
  // 데이터베이스 디렉토리 생성
  if let Some(parent) = db_path.parent() {
      std::fs::create_dir_all(parent)?;
  }
  
  // 새 데이터베이스 생성 및 초기화
  let db = Database::new(&db_path)?;
  db.initialize()?;
  
  println!("{} 데이터베이스를 초기화했습니다!", "🎉".green());
  println!("  경로: {}", db_path.display().to_string().cyan());
  
  Ok(())
}

fn print_todos_table(todos: &[Todo]) {
  let mut table = Table::new();
  
  // 헤더 설정
  table.add_row(Row::new(vec![
    Cell::new("ID").style_spec("bFc"),
    Cell::new("상태").style_spec("bFc"),
    Cell::new("우선순위").style_spec("bFc"),
    Cell::new("제목").style_spec("bFc"),
    Cell::new("마감일").style_spec("bFc"),
    Cell::new("생성일").style_spec("bFc"),
  ]));
  
  // 데이터 행 추가
  for todo in todos {
    let id_cell = Cell::new(&todo.id.unwrap_or(0).to_string());
    
    let status_cell = Cell::new(&format!("{} {}", 
      todo.status.to_emoji(), 
      todo.status.to_display_string()
    ));
    
    let priority_cell = Cell::new(&format!("{} {}", 
      todo.priority.to_emoji(), 
      todo.priority.to_display_string()
    )).style_spec(match todo.priority {
      Priority::High => "Fr",
      Priority::Medium => "Fy",
      Priority::Low => "Fg",
    });
    
    let title_cell = if todo.status == Status::Done {
      Cell::new(&todo.title).style_spec("Fd")
    } else if todo.is_overdue() {
      Cell::new(&format!("⚠️  {}", todo.title)).style_spec("Fr")
    } else {
      Cell::new(&todo.title)
    };
    
    let due_cell = if let Some(due) = todo.due_date {
      let formatted = utils::format_date(&due);
      if todo.is_overdue() {
        Cell::new(&formatted).style_spec("Fr")
      } else if let Some(days) = todo.days_until_due() {
        if days <= 1 {
          Cell::new(&format!("{} ({}일 후)", formatted, days)).style_spec("Fy")
        } else {
          Cell::new(&format!("{} ({}일 후)", formatted, days))
        }
      } else {
        Cell::new(&formatted)
      }
    } else {
      Cell::new("-")
    };
    
    let created_cell = Cell::new(&utils::format_date(&todo.created_at));
    
    table.add_row(Row::new(vec![
      id_cell,
      status_cell,
      priority_cell,
      title_cell,
      due_cell,
      created_cell,
    ]));
  }
  
  table.printstd();
}