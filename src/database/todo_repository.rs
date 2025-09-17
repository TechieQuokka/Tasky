use chrono::{Utc};
use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::error::{Result};
use crate::models::{
  CreateTodo, Priority, SortBy, SortOrder, Status, Todo, TodoFilter, TodoStats, UpdateTodo,
};

pub trait TodoRepository {
  fn create(&self, todo: &CreateTodo) -> Result<Todo>;
  fn find_by_id(&self, id: i64) -> Result<Option<Todo>>;
  fn find_all(&self, filter: &TodoFilter, sort_by: SortBy, sort_order: SortOrder) -> Result<Vec<Todo>>;
  fn update(&self, id: i64, todo: &UpdateTodo) -> Result<Option<Todo>>;
  fn delete(&self, id: i64) -> Result<bool>;
  fn get_stats(&self) -> Result<TodoStats>;
}

pub struct SqliteTodoRepository<'a> {

  conn: &'a Connection,
}

impl<'a> SqliteTodoRepository<'a> {
  pub fn new(conn: &'a Connection) -> Self {
    Self { conn }
  }

  fn row_to_todo(row: &Row) -> rusqlite::Result<Todo> {
    Ok(Todo {
      id: Some(row.get(0)?),
      title: row.get(1)?,
      description: row.get(2)?,
      priority: match row.get::<_, i32>(3)? {
        0 => Priority::Low,
        1 => Priority::Medium,
        2 => Priority::High,
        _ => Priority::Medium,
      },
      status: match row.get::<_, i32>(4)? {
        0 => Status::Pending,
        1 => Status::Done,
        _ => Status::Pending,
      },
      created_at: row.get(5)?,
      updated_at: row.get(6)?,
      due_date: row.get(7)?,
    })
  }

  fn build_filter_clause(filter: &TodoFilter) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {

    let mut conditions = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(status) = filter.status {
      conditions.push("status = ?".to_string());
      params.push(Box::new(status as i32));
    }

    if let Some(priority) = filter.priority {
      conditions.push("priority = ?".to_string());
      params.push(Box::new(priority as i32));
    }

    if let Some(created_before) = filter.created_before {
      conditions.push("created_at < ?".to_string());
      params.push(Box::new(created_before));
    }

    if let Some(created_after) = filter.created_after {
      conditions.push("created_at > ?".to_string());
      params.push(Box::new(created_after));
    }

    if let Some(due_before) = filter.due_before {
      conditions.push("due_date < ?".to_string());
      params.push(Box::new(due_before));
    }

    if let Some(due_after) = filter.due_after {
      conditions.push("due_date > ?".to_string());
      params.push(Box::new(due_after));
    }

    let where_clause = if conditions.is_empty() {
      String::new()
    } else {
      format!("WHERE {}", conditions.join(" AND "))
    };

    (where_clause, params)
  }

  fn build_order_clause(sort_by: SortBy, sort_order: SortOrder) -> String {
    let column = match sort_by {
      SortBy::CreatedAt => "created_at",
      SortBy::UpdatedAt => "updated_at",
      SortBy::DueDate => "due_date",
      SortBy::Priority => "priority",
      SortBy::Title => "title",
    };

    let order = match sort_order {
      SortOrder::Asc => "ASC",
      SortOrder::Desc => "DESC",
    };

    format!("ORDER BY {} {}", column, order)
  }
}

impl<'a> TodoRepository for SqliteTodoRepository<'a> {
  fn create(&self, todo: &CreateTodo) -> Result<Todo> {
    let now = Utc::now();

    self.conn.execute(
        "INSERT INTO todos (title, description, priority, status, created_at, updated_at, due_date)
          VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            todo.title,
            todo.description,
            todo.priority as i32,
            Status::Pending as i32,
            now,
            now,
            todo.due_date,
        ],
    )?;

    let id = self.conn.last_insert_rowid();
    Ok(Todo {
      id: Some(id),
      title: todo.title.clone(),
      description: todo.description.clone(),
      priority: todo.priority,
      status: Status::Pending,
      created_at: now,
      updated_at: now,
      due_date: todo.due_date,
    })
  }

  fn find_by_id(&self, id: i64) -> Result<Option<Todo>> {
    let mut stmt = self.conn.prepare(
      "SELECT id, title, description, priority, status, created_at, updated_at, due_date 
            FROM todos WHERE id = ?1"
    )?;

    stmt.query_row(params![id], Self::row_to_todo)
        .optional()
        .map_err(|e| e.into())
  }

  fn find_all(&self, filter: &TodoFilter, sort_by: SortBy, sort_order: SortOrder) -> Result<Vec<Todo>> {
    let (where_clause, params) = Self::build_filter_clause(filter);
    let order_clause = Self::build_order_clause(sort_by, sort_order);

    let query = format!(
      "SELECT id, title, description, priority, status, created_at, updated_at, due_date 
            FROM todos {} {}", where_clause, order_clause
    );

    let mut stmt = self.conn.prepare(&query)?;
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let todos = stmt
        .query_map(&param_refs[..], Self::row_to_todo)?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(todos)
  }

  fn update(&self, id: i64, todo: &UpdateTodo) -> Result<Option<Todo>> {
      
    let existing = self.find_by_id(id)?;

    if let Some(mut existing_todo) = existing {
      if let Some(title) = &todo.title {
        existing_todo.title = title.clone();
      }
      if let Some(description) = &todo.description {
        existing_todo.description = Some(description.clone());
      }
      if let Some(priority) = todo.priority {
        existing_todo.priority = priority;
      }
      if let Some(due_date) = todo.due_date {
        existing_todo.due_date = Some(due_date);
      }

      existing_todo.updated_at = Utc::now();

      // 데이터베이스 업데이트
      self.conn.execute(
        "UPDATE todos SET title = ?1, description = ?2, priority = ?3, 
          status = ?4, due_date = ?5, updated_at = ?6 WHERE id = ?7",
        params![
          existing_todo.title,
          existing_todo.description,
          existing_todo.priority as i32,
          existing_todo.status as i32,
          existing_todo.due_date,
          existing_todo.updated_at,
          id
        ],
      )?;

      Ok(Some(existing_todo))
    } else {
      Ok(None)
    }
  }

  fn delete(&self, id: i64) -> Result<bool> {
    let affected = self.conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
    Ok(affected > 0)
  }

  fn get_stats(&self) -> Result<TodoStats> {
    let total_todos: i64 = self.conn.query_row(
      "SELECT COUNT(*) FROM todos",
      [],
      |row| row.get(0),
    )?;

    let pending_todos: i64 = self.conn.query_row(
      "SELECT COUNT(*) FROM todos WHERE status = 0",
      [],
      |row| row.get(0),
    )?;

    let completed_todos: i64 = self.conn.query_row(
      "SELECT COUNT(*) FROM todos WHERE status = 1",
      [],
      |row| row.get(0),
    )?;

    let high_priority_todos: i64 = self.conn.query_row(
      "SELECT COUNT(*) FROM todos WHERE priority = 2",
      [],
      |row| row.get(0),
    )?;

    let now = Utc::now();
    let overdue_todos: i64 = self.conn.query_row(
      "SELECT COUNT(*) FROM todos WHERE status = 0 AND due_date < ?1 AND due_date IS NOT NULL",
      params![now],
      |row| row.get(0),
    )?;

    let completion_rate = if total_todos > 0 {
      (completed_todos as f64 / total_todos as f64) * 100.0
    } else {
      0.0
    };

    Ok(TodoStats {
      total_todos,
      pending_todos,
      completed_todos,
      high_priority_todos,
      overdue_todos,
      completion_rate,
    })
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[test]
    fn test_create_and_find() {
        let db = Database::in_memory().unwrap();
        db.initialize().unwrap();
        let repo = SqliteTodoRepository::new(db.conn());

        let create_todo = CreateTodo::new("테스트 할일".to_string());
        let created = repo.create(&create_todo).unwrap();
        
        assert!(created.id.is_some());
        assert_eq!(created.title, "테스트 할일");
        
        let found = repo.find_by_id(created.id.unwrap()).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "테스트 할일");
    }

    #[test]
    fn test_update() {
        let db = Database::in_memory().unwrap();
        db.initialize().unwrap();
        let repo = SqliteTodoRepository::new(db.conn());

        let create_todo = CreateTodo::new("원래 제목".to_string());
        let created = repo.create(&create_todo).unwrap();
        let id = created.id.unwrap();

        let mut update_todo = UpdateTodo::default();
        update_todo.title = Some("수정된 제목".to_string());
        
        let updated = repo.update(id, &update_todo).unwrap();
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().title, "수정된 제목");
    }

    #[test]
    fn test_delete() {
        let db = Database::in_memory().unwrap();
        db.initialize().unwrap();
        let repo = SqliteTodoRepository::new(db.conn());

        let create_todo = CreateTodo::new("삭제될 할일".to_string());
        let created = repo.create(&create_todo).unwrap();
        let id = created.id.unwrap();

        let deleted = repo.delete(id).unwrap();
        assert!(deleted);

        let found = repo.find_by_id(id).unwrap();
        assert!(found.is_none());
    }
}