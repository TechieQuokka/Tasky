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
    Commands::List { status, priority, sort, order, today, overdue, urgent, verbose } => {
      handle_list(&service, status, priority, sort, order, today, overdue, urgent, verbose)
    }
    Commands::Show { id } => {
      handle_show(&service, id)
    }
    Commands::Done { ids } => {
      handle_done(&service, ids)
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
    Commands::DbInfo => {
      handle_db_info()
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
  verbose: bool,
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

  if verbose {
    print_todos_verbose(&todos);
  } else {
    print_todos_table(&todos);
  }
  println!("\n총 {}개의 할일", todos.len().to_string().cyan());

  Ok(())
}

fn handle_show(service: &impl TodoService, id: i64) -> Result<()> {
  let todo = service.get_todo_by_id(id)?;

  println!("\n{}", "📋 할일 상세 정보".bold().blue());
  println!("{}", "─".repeat(50));

  println!("ID: {}", todo.id.unwrap_or(0).to_string().cyan());
  println!("제목: {}", todo.title.bold());

  if let Some(desc) = &todo.description {
    if !desc.trim().is_empty() {
      println!("설명: {}", desc);
    } else {
      println!("설명: {}", "없음".dimmed());
    }
  } else {
    println!("설명: {}", "없음".dimmed());
  }

  println!("상태: {} {}", todo.status.to_emoji(), todo.status.to_display_string());
  println!("우선순위: {} {}", todo.priority.to_emoji(), todo.priority.to_display_string());

  if let Some(due) = todo.due_date {
    println!("마감일: {}", utils::format_date(&due).yellow());
    if let Some(days) = todo.days_until_due() {
      if days == 0 {
        println!("⚠️  {}", "오늘이 마감일입니다!".red().bold());
      } else if days < 0 {
        println!("⚠️  {}일 지났습니다", (-days).to_string().red().bold());
      } else {
        println!("남은 일수: {}일", days.to_string().green());
      }
    }
  } else {
    println!("마감일: {}", "설정되지 않음".dimmed());
  }

  println!("생성일: {}", utils::format_date(&todo.created_at));
  println!("수정일: {}", utils::format_date(&todo.updated_at));

  println!("{}", "─".repeat(50));

  Ok(())
}

fn handle_done (service: &impl TodoService, ids: Vec<i64>) -> Result<()> {
  if ids.is_empty() {
    println!("{} 완료할 할일 ID를 입력해주세요.", "⚠️".yellow());
    return Ok(());
  }

  let mut completed_count = 0;
  let mut errors = Vec::new();

  for id in ids {
    match service.complete_todo(id) {
      Ok(todo) => {
        println!("{} 할일을 완료했습니다!", "✅".green());
        println!("  ID: {}, 제목: {}", id.to_string().cyan(), todo.title.strikethrough());
        completed_count += 1;
      }
      Err(e) => {
        errors.push((id, e));
      }
    }
  }

  if !errors.is_empty() {
    println!("\n{} 완료 실패한 할일:", "❌".red());
    for (id, error) in errors {
      println!("  ID {}: {}", id.to_string().cyan(), error);
    }
  }

  if completed_count > 0 {
    println!("\n총 {}개의 할일을 완료했습니다.", completed_count.to_string().green());
  }

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
  use std::thread;
  use std::time::Duration;

  let db_path = Database::default_path();

  if db_path.exists() && !force {
      println!("{} 데이터베이스가 이미 존재합니다.", "⚠️".yellow());
      println!("기존 데이터베이스를 삭제하고 새로 만들려면 --force 옵션을 사용하세요.");
      return Ok(());
  }

  if db_path.exists() && force {
      println!("{} 기존 데이터베이스를 삭제하는 중...", "🗑️".yellow());

      // 파일 잠금 문제를 해결하기 위한 안전한 삭제 시도
      let mut attempts = 0;
      const MAX_ATTEMPTS: u32 = 5;

      while attempts < MAX_ATTEMPTS {
          match try_remove_database_safely(&db_path) {
              Ok(_) => {
                  println!("{} 기존 데이터베이스를 삭제했습니다.", "✅".green());
                  break;
              }
              Err(e) => {
                  attempts += 1;
                  if attempts >= MAX_ATTEMPTS {
                      // 마지막 시도에서도 실패하면 백업 전략 사용
                      return handle_init_with_backup_strategy(&db_path);
                  }

                  println!("{} 삭제 시도 {}/{} 실패: {}",
                      "⏳".yellow(), attempts, MAX_ATTEMPTS, e);

                  // 잠시 대기 후 재시도
                  thread::sleep(Duration::from_millis(500));
              }
          }
      }
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

fn try_remove_database_safely(db_path: &std::path::Path) -> Result<()> {
    use std::fs;

    // 1. 파일이 읽기 전용인지 확인하고 권한 변경
    if let Ok(metadata) = fs::metadata(db_path) {
        if metadata.permissions().readonly() {
            let mut perms = metadata.permissions();
            perms.set_readonly(false);
            fs::set_permissions(db_path, perms)?;
        }
    }

    // 2. 관련 SQLite 파일들도 함께 삭제 시도
    let extensions = ["-wal", "-shm", "-journal"];
    for ext in &extensions {
        let related_file = db_path.with_extension(format!("db{}", ext));
        if related_file.exists() {
            let _ = fs::remove_file(&related_file); // 오류 무시 - 메인 파일이 중요
        }
    }

    // 3. 메인 데이터베이스 파일 삭제
    fs::remove_file(db_path)?;

    Ok(())
}

fn handle_init_with_backup_strategy(db_path: &std::path::Path) -> Result<()> {
    use std::fs;

    println!("{} 직접 삭제가 불가능합니다. 백업 전략을 사용합니다.", "⚠️".yellow());

    // 사용자에게 도움말 제공
    print_database_lock_help();

    // 1. 임시 이름으로 기존 파일 이동
    let backup_path = db_path.with_extension("db.backup");
    let mut counter = 1;
    let mut final_backup_path = backup_path.clone();

    while final_backup_path.exists() {
        final_backup_path = db_path.with_extension(format!("db.backup.{}", counter));
        counter += 1;
    }

    match fs::rename(db_path, &final_backup_path) {
        Ok(_) => {
            println!("{} 기존 파일을 {}로 이동했습니다.",
                "📁".blue(), final_backup_path.display());
        }
        Err(_) => {
            // 이동도 실패하면 원본 경로를 유지하면서 새로운 데이터베이스 생성
            println!("{} 파일 이동이 불가능합니다. 원본 파일을 유지하고 계속 진행합니다.", "⚠️".yellow());

            // 기존 파일을 그대로 두고 새로운 연결로 시도
            match try_create_database_with_existing_file(db_path) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    // 마지막 수단: 임시 경로에 생성
                    println!("{} 대체 경로에 새 데이터베이스를 생성합니다.", "💡".blue());
                    let temp_path = db_path.with_extension("db.new");
                    return create_database_at_path(&temp_path);
                }
            }
        }
    }

    // 2. 새 데이터베이스 생성
    create_database_at_path(db_path)
}

fn print_database_lock_help() {
    println!("\n{}", "💡 데이터베이스 파일 잠금 해결 방법:".bold().blue());
    println!("  1. 실행 중인 다른 tasky 프로세스를 종료하세요");
    println!("  2. Windows 작업 관리자에서 tasky.exe 프로세스를 찾아 종료하세요");
    println!("  3. SQLite 브라우저나 DB 관리 도구가 파일을 열고 있다면 닫으세요");
    println!("  4. 바이러스 백신이 파일을 스캔 중일 수 있으니 잠시 기다려보세요\n");
}

fn try_create_database_with_existing_file(db_path: &std::path::Path) -> Result<()> {
    use crate::database::Database;

    // 기존 파일이 있어도 새로운 연결로 덮어쓰기 시도
    match Database::new(db_path) {
        Ok(db) => {
            // 테이블을 드롭하고 재생성
            let _ = db.conn().execute_batch("
                DROP TABLE IF EXISTS todos;
                DROP TABLE IF EXISTS sqlite_sequence;
            ");

            db.initialize()?;
            println!("{} 기존 데이터베이스를 재초기화했습니다!", "🎉".green());
            println!("  경로: {}", db_path.display().to_string().cyan());
            Ok(())
        }
        Err(e) => Err(e)
    }
}

fn create_database_at_path(db_path: &std::path::Path) -> Result<()> {
    use crate::database::Database;

    // 데이터베이스 디렉토리 생성
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 새 데이터베이스 생성 및 초기화
    let db = Database::new(db_path)?;
    db.initialize()?;

    println!("{} 데이터베이스를 초기화했습니다!", "🎉".green());
    println!("  경로: {}", db_path.display().to_string().cyan());

    Ok(())
}

fn handle_db_info() -> Result<()> {
    use crate::database::Database;

    let db_path = Database::default_path();

    println!("{}", "📊 데이터베이스 정보".bold().blue());
    println!("{}", "─".repeat(50));

    // 환경변수 정보 표시
    match std::env::var("TASKY_DB_PATH") {
        Ok(custom_path) => {
            println!("환경변수: {} 설정됨", "TASKY_DB_PATH".green());
            println!("커스텀 경로: {}", custom_path.cyan());
        }
        Err(_) => {
            println!("환경변수: {} 기본 경로 사용", "TASKY_DB_PATH".yellow());
        }
    }

    println!("실제 경로: {}", db_path.display().to_string().cyan());

    if !db_path.exists() {
        println!("상태: {} 데이터베이스 파일이 존재하지 않습니다", "❌".red());
        println!("{} 다음 명령어로 데이터베이스를 생성하세요:", "💡".yellow());
        println!("  tasky init");
        return Ok(());
    }

    // 파일 크기 확인
    if let Ok(metadata) = std::fs::metadata(&db_path) {
        let size = metadata.len();
        println!("크기: {} bytes", size.to_string().green());

        let modified = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        if let Ok(duration) = modified.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                .unwrap_or_default();
            println!("수정일: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string().yellow());
        }
    }

    // 데이터베이스 연결 시도
    match Database::new(&db_path) {
        Ok(db) => {
            println!("연결: {} 성공", "✅".green());

            if db.is_initialized() {
                println!("초기화: {} 완료", "✅".green());

                // 테이블 정보 확인
                match db.conn().prepare("SELECT COUNT(*) FROM todos") {
                    Ok(mut stmt) => {
                        if let Ok(count) = stmt.query_row([], |row| row.get::<_, i64>(0)) {
                            println!("할일 개수: {}", count.to_string().cyan());
                        }
                    }
                    Err(_) => {
                        println!("할일 개수: {} 조회 실패", "❌".red());
                    }
                }
            } else {
                println!("초기화: {} 미완료", "❌".red());
                println!("{} 다음 명령어로 데이터베이스를 초기화하세요:", "💡".yellow());
                println!("  tasky init --force");
            }
        }
        Err(e) => {
            println!("연결: {} 실패", "❌".red());
            println!("오류: {}", e.to_string().red());

            if e.to_string().contains("database is locked") ||
               e.to_string().contains("다른 프로세스가 파일을 사용") {
                print_database_lock_help();
            }
        }
    }

    // 관련 파일들 확인
    let related_files = ["-wal", "-shm", "-journal"];
    let mut found_related = false;

    for ext in &related_files {
        let related_path = db_path.with_extension(format!("db{}", ext));
        if related_path.exists() {
            if !found_related {
                println!("\n{}", "관련 파일:".bold());
                found_related = true;
            }
            if let Ok(metadata) = std::fs::metadata(&related_path) {
                println!("  {} ({} bytes)", related_path.display(), metadata.len());
            }
        }
    }

    println!("{}", "─".repeat(50));

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
      if let Some(days) = todo.days_until_due() {
        if days < 0 {
          Cell::new(&format!("{} ({}일 전)", formatted, -days)).style_spec("Fr")
        } else if days <= 1 {
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

fn print_todos_verbose(todos: &[Todo]) {
  for (index, todo) in todos.iter().enumerate() {
    if index > 0 {
      println!("{}", "─".repeat(60).dimmed());
    }

    println!("ID: {} | 상태: {} {} | 우선순위: {} {}",
      todo.id.unwrap_or(0).to_string().cyan(),
      todo.status.to_emoji(),
      todo.status.to_display_string(),
      todo.priority.to_emoji(),
      todo.priority.to_display_string()
    );

    if todo.status == Status::Done {
      println!("제목: {}", todo.title.strikethrough());
    } else if todo.is_overdue() {
      println!("제목: ⚠️  {}", todo.title.red());
    } else {
      println!("제목: {}", todo.title.bold());
    }

    if let Some(desc) = &todo.description {
      if !desc.trim().is_empty() {
        println!("설명: {}", desc);
      }
    }

    if let Some(due) = todo.due_date {
      let formatted = utils::format_date(&due);
      if let Some(days) = todo.days_until_due() {
        if days < 0 {
          println!("마감일: {} {} ({}일 전)", "⚠️".red(), formatted.red(), -days);
        } else if days <= 1 {
          println!("마감일: {} ({}일 후)", formatted.yellow(), days);
        } else {
          println!("마감일: {} ({}일 후)", formatted, days);
        }
      } else {
        println!("마감일: {}", formatted);
      }
    }

    println!("생성일: {}", utils::format_date(&todo.created_at).dimmed());
  }
}