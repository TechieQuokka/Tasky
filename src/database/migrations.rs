use rusqlite::Connection;

use crate::error::Result;

pub fn run_migrations(conn: &Connection) -> Result<()> {

  create_todos_table(conn)?;
  create_indexes(conn)?;
  Ok(())
}

fn create_todos_table(conn: &Connection) -> Result<()> {

  conn.execute(
    "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            priority INTEGER NOT NULL DEFAULT 1,
            status INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            due_date TEXT,
            CHECK (priority IN (0, 1, 2)),
            CHECK (status IN (0, 1))
        )", [])?;
  Ok(())
}

fn create_indexes(conn: &Connection) -> Result<()> {

  // 상태별 조회 최적화
  conn.execute(
    "CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status)",
    [],
  )?;

  // 우선순위별 조회 최적화
  conn.execute(
    "CREATE INDEX IF NOT EXISTS idx_todos_priority ON todos(priority)",
    [],
  )?;

  // 마감일별 조회 최적화
  conn.execute(
    "CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date)",
    [],
  )?;

  // 생성일별 정렬 최적화
  conn.execute(
    "CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos(created_at)",
    [],
  )?;

  Ok(())
}

/// 데이터베이스 리셋 (테스트용)
#[cfg(test)]
pub fn reset_database(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS todos", [])?;
    run_migrations(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use rusqlite::Connection;

  #[test]
  fn test_create_todos_table() {
    let conn = Connection::open_in_memory().unwrap();
    let result = create_todos_table(&conn);
    assert!(result.is_ok());

    // 테이블이 생성되었는지 확인
    let count: i32 = conn
      .query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='todos'",
        [],
        |row| row.get(0),
      )
      .unwrap();
    assert_eq!(count, 1);
  }

  #[test]
  fn test_run_migrations() {
    let conn = Connection::open_in_memory().unwrap();
    let result = run_migrations(&conn);
    assert!(result.is_ok());

    // 마이그레이션을 다시 실행해도 오류가 없어야 함
    let result = run_migrations(&conn);
    assert!(result.is_ok());
  }
}