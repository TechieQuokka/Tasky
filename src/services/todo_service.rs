use crate::database::{Database, SqliteTodoRepository, TodoRepository};
use crate::error::{Result, TaskyError};
use crate::models::{
  CreateTodo, Priority, SortBy, SortOrder, Status, Todo, TodoFilter, TodoStats, UpdateTodo,
};
use crate::utils;

pub trait TodoService {
    
  fn create_todo(&self, create_todo: CreateTodo) -> Result<Todo>;
  fn get_todo_by_id(&self, id: i64) -> Result<Todo>;
  fn list_todos(&self, filter: Option<TodoFilter>, sort_by: Option<SortBy>, sort_order: Option<SortOrder>,) -> Result<Vec<Todo>>;
  fn update_todo(&self, id: i64, update_todo: UpdateTodo) -> Result<Todo>;
  fn delete_todo(&self, id: i64) -> Result<bool>;
  fn complete_todo(&self, id: i64) -> Result<Todo>;
  fn uncomplete_todo(&self, id: i64) -> Result<Todo>;
  fn get_stats(&self) -> Result<TodoStats>;
  fn get_today_todos(&self) -> Result<Vec<Todo>>;
  fn get_urgent_todos(&self) -> Result<Vec<Todo>>;
  fn get_overdue_todos(&self) -> Result<Vec<Todo>>;
}

pub struct DefaultTodoService {
  db: Database,
}

impl DefaultTodoService {
  pub fn new(db: Database) -> Self {
    Self { db }
  }

  pub fn default() -> Result<Self> {
    let db = Database::default()?;

    if !db.is_initialized() {
      db.initialize()?;
    }

    Ok(Self::new(db))
  }

  pub fn in_memory() -> Result<Self> {
    let db = Database::in_memory()?;
    db.initialize()?;
    Ok(Self::new(db))
  }

  fn validate_create_todo(&self, todo: &CreateTodo) -> Result<()> {

    if todo.title.trim().is_empty() {
      return Err(TaskyError::EmptyTitle);
    }

    if todo.title.len() > 200 {
      return Err(TaskyError::InvalidInput { message: "제목은 200자를 초과할 수 없습니다.".to_string(), });
    }

    if let Some(desc) = &todo.description {
      if desc.len() > 1000 {
        return Err(TaskyError::InvalidInput { message: "설명은 1000자를 초과할 수 없습니다.".to_string(), });
      }
    }

    Ok(())
  }

  fn validate_update_todo(&self, todo: &UpdateTodo) -> Result<()> {

    if let Some(title) = &todo.title {
      if title.trim().is_empty() {
        return Err(TaskyError::EmptyTitle);
      }

      if title.len() > 200 {
        return Err(TaskyError::InvalidInput { message: "제목은 200자를 초과할 수 없습니다.".to_string(), });
      }
    }

    if let Some(desc) = &todo.description {
      if desc.len() > 1000 {
        return Err(TaskyError::InvalidInput { message: "설명은 1000자를 초과할 수 없습니다.".to_string(), });
      }
    }

    Ok(())
  }
}

impl TodoService for DefaultTodoService {
  fn create_todo(&self, mut create_todo: CreateTodo) -> Result<Todo> {
    self.validate_create_todo(&create_todo)?;

    create_todo.title = create_todo.title.trim().to_string();

    if let Some(desc) = create_todo.description {
      create_todo.description = Some(desc.trim().to_string());
    }

    let repo = SqliteTodoRepository::new(self.db.conn());
    repo.create(&create_todo)
  }

  fn get_todo_by_id(&self, id: i64) -> Result<Todo> {
    
    let repo = SqliteTodoRepository::new(self.db.conn());
    repo.find_by_id(id)?
        .ok_or(TaskyError::TodoNotFound { id })
  }

  fn list_todos(&self, filter: Option<TodoFilter>, sort_by: Option<SortBy>, sort_order: Option<SortOrder>,) -> Result<Vec<Todo>> {
      
    let repo = SqliteTodoRepository::new(self.db.conn());
    let filter = filter.unwrap_or_default();
    let sort_by = sort_by.unwrap_or_default();
    let sort_order = sort_order.unwrap_or_default();

    repo.find_all(&filter, sort_by, sort_order)
  }

  fn update_todo(&self, id: i64, mut update_todo: UpdateTodo) -> Result<Todo> {
    self.validate_update_todo(&update_todo)?;

    if let Some(title) = update_todo.title {
      update_todo.title = Some(title.trim().to_string());
    }

    if let Some(desc) = update_todo.description {
      update_todo.description = Some(desc.trim().to_string());
    }

    let repo = SqliteTodoRepository::new(self.db.conn());

    repo.update(id, &update_todo)?
        .ok_or(TaskyError::TodoNotFound { id })
  }

  fn delete_todo(&self, id: i64) -> Result<bool> {
    let repo = SqliteTodoRepository::new(self.db.conn());

    let deleted = repo.delete(id)?;
    if !deleted {
      return Err(TaskyError::TodoNotFound { id });
    }

    Ok(deleted)
  }

  fn complete_todo(&self, id: i64) -> Result<Todo> {
    let mut update = UpdateTodo::default();
    update.status = Some(Status::Done);

    self.update_todo(id, update)
  }

  fn uncomplete_todo(&self, id: i64) -> Result<Todo> {
      let mut update = UpdateTodo::default();
      update.status = Some(Status::Pending);

      self.update_todo(id, update)
  }

  fn get_stats(&self) -> Result<TodoStats> {
      let repo = SqliteTodoRepository::new(self.db.conn());
      repo.get_stats()
  }

  fn get_today_todos(&self) -> Result<Vec<Todo>> {
      let mut filter = TodoFilter::default();
      filter.due_after = Some(utils::today_start());
      filter.due_before = Some(utils::today_end());

      self.list_todos(Some(filter), Some(SortBy::DueDate), Some(SortOrder::Asc))
  }

  fn get_urgent_todos(&self) -> Result<Vec<Todo>> {
      let mut filter = TodoFilter::default();
      filter.status = Some(Status::Pending);
      filter.priority = Some(Priority::High);

      self.list_todos(Some(filter), Some(SortBy::DueDate), Some(SortOrder::Asc))
  }

  fn get_overdue_todos(&self) -> Result<Vec<Todo>> {
      let today_start = crate::utils::today_start();

      let mut filter = TodoFilter::default();
      filter.status = Some(Status::Pending);
      filter.due_before = Some(today_start);

      self.list_todos(Some(filter), Some(SortBy::DueDate), Some(SortOrder::Asc))
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_create_todo() {
        let service = DefaultTodoService::in_memory().unwrap();
        
        let create = CreateTodo::new("테스트 할일".to_string());
        let todo = service.create_todo(create).unwrap();
        
        assert_eq!(todo.title, "테스트 할일");
        assert_eq!(todo.status, Status::Pending);
        assert_eq!(todo.priority, Priority::Medium);
    }
    
    #[test]
    fn test_complete_todo() {
        let service = DefaultTodoService::in_memory().unwrap();
        
        // 할일 생성
        let create = CreateTodo::new("완료할 할일".to_string());
        let todo = service.create_todo(create).unwrap();
        let id = todo.id.unwrap();
        
        // 완료 처리
        let completed = service.complete_todo(id).unwrap();
        assert_eq!(completed.status, Status::Done);
        
        // 미완료 처리
        let uncompleted = service.uncomplete_todo(id).unwrap();
        assert_eq!(uncompleted.status, Status::Pending);
    }
    
    #[test]
    fn test_validate_empty_title() {
        let service = DefaultTodoService::in_memory().unwrap();
        
        let create = CreateTodo::new("   ".to_string()); // 공백만 있는 제목
        let result = service.create_todo(create);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TaskyError::EmptyTitle));
    }
    
    #[test]
    fn test_delete_todo() {
        let service = DefaultTodoService::in_memory().unwrap();
        
        // 할일 생성
        let create = CreateTodo::new("삭제할 할일".to_string());
        let todo = service.create_todo(create).unwrap();
        let id = todo.id.unwrap();
        
        // 삭제
        let deleted = service.delete_todo(id).unwrap();
        assert!(deleted);
        
        // 다시 조회하면 없어야 함
        let result = service.get_todo_by_id(id);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_get_stats() {
        let service = DefaultTodoService::in_memory().unwrap();
        
        // 여러 할일 생성
        service.create_todo(CreateTodo::new("할일 1".to_string())
            .with_priority(Priority::High)).unwrap();
        
        service.create_todo(CreateTodo::new("할일 2".to_string())).unwrap();
        
        let todo3 = service.create_todo(CreateTodo::new("할일 3".to_string())).unwrap();
        service.complete_todo(todo3.id.unwrap()).unwrap();
        
        // 통계 확인
        let stats = service.get_stats().unwrap();
        assert_eq!(stats.total_todos, 3);
        assert_eq!(stats.pending_todos, 2);
        assert_eq!(stats.completed_todos, 1);
        assert_eq!(stats.high_priority_todos, 1);
    }
    
    #[test]
    fn test_overdue_todos() {
        let service = DefaultTodoService::in_memory().unwrap();

        // 어제 마감인 할일 생성
        let yesterday = Utc::now() - Duration::days(1);
        let create_overdue = CreateTodo::new("늦은 할일".to_string())
            .with_due_date(yesterday);
        service.create_todo(create_overdue).unwrap();

        // 오늘 마감인 할일 생성 (overdue에 포함되지 않아야 함)
        let today = crate::utils::today_start();
        let create_today = CreateTodo::new("오늘 할일".to_string())
            .with_due_date(today);
        service.create_todo(create_today).unwrap();

        // 내일 마감인 할일 생성 (overdue에 포함되지 않아야 함)
        let tomorrow = Utc::now() + Duration::days(1);
        let create_future = CreateTodo::new("내일 할일".to_string())
            .with_due_date(tomorrow);
        service.create_todo(create_future).unwrap();

        // 기한 지난 할일 조회 (어제 것만 포함되어야 함)
        let overdue = service.get_overdue_todos().unwrap();
        assert_eq!(overdue.len(), 1);
        assert_eq!(overdue[0].title, "늦은 할일");

        // 오늘 할일 조회
        let today_todos = service.get_today_todos().unwrap();
        assert_eq!(today_todos.len(), 1);
        assert_eq!(today_todos[0].title, "오늘 할일");
    }
}