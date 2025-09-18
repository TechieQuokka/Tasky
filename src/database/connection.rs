use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::error::TaskyError;

pub struct Database {
  conn: Connection,
}

impl Database {

  pub fn new(path: &Path) -> crate::Result<Self> {
    let conn = Connection::open(path)?;

    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    Ok(Database { conn })
  }

  pub fn default() -> crate::Result<Self> {
    let db_path = Self::default_path();

    if let Some(parent) = db_path.parent() {
      std::fs::create_dir_all(parent).map_err(|e| TaskyError::Io(e))?;
    }

    Self::new(&db_path)
  }

  pub fn in_memory() -> crate::Result<Self> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(Database { conn })
  }

  pub fn default_path() -> PathBuf {
    // Windows: %APPDATA%/tasky/tasky.db
    // Linux/Mac: ~/.local/share/tasky/tasky.db
    if let Some(data_dir) = dirs::data_dir() {
      data_dir.join("tasky").join("tasky.db")
    } else {
      PathBuf::from("tasky.db")
    }
  }

  pub fn conn(&self) -> &Connection {
    &self.conn
  }

  pub fn conn_mut(&mut self) -> &mut Connection {
    &mut self.conn
  }

  pub fn initialize(&self) -> crate::Result<()> {
    crate::database::migrations::run_migrations(&self.conn)?;
    Ok(())
  }

  pub fn is_initialized(&self) -> bool {
    let mut stmt = match self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='todos'") {
      Ok(stmt) => stmt,
      Err(_) => return false,
    };

    match stmt.query_row([], |_| Ok(())) {
      Ok(_) => true,
      Err(_) => false,
    }
  }

  pub fn transaction<F, R>(&mut self, f: F) -> crate::Result<R>
      where F: FnOnce(&rusqlite::Transaction) -> crate::Result<R>,
  {
    let tx = self.conn.transaction()?;
    let result = f(&tx)?;
    tx.commit()?;
    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_in_memory_database() {
    let db = Database::in_memory();
    assert!(db.is_ok());
  }

  #[test]
  fn test_default_path() {
    let path = Database::default_path();
    assert!(path.to_string_lossy().contains("tasky"));
  }
}