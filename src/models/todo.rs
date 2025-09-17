use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{Result, TaskyError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {

  pub id: Option<i64>,
  pub title: String,
  pub description: Option<String>,
  pub priority: Priority,
  pub status: Status,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub due_date: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
  Low = 0,
  Medium = 1,
  High = 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
  Pending = 0,
  Done = 1
}

#[derive(Debug, Clone)]
pub struct CreateTodo {
  pub title: String,
  pub description: Option<String>,
  pub priority: Priority,
  pub due_date: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTodo {
  pub title: Option<String>,
  pub description: Option<String>,
  pub priority: Option<Priority>,
  pub status: Option<Status>,
  pub due_date: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Default)]
pub struct TodoFilter {
  pub status: Option<Status>,
  pub priority: Option<Priority>,
  pub created_before: Option<DateTime<Utc>>,
  pub created_after: Option<DateTime<Utc>>,
  pub due_before: Option<DateTime<Utc>>,
  pub due_after: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
  CreatedAt,
  UpdatedAt,
  DueDate,
  Priority,
  Title
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
  Asc, Desc
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoStats {
  pub total_todos: i64,
  pub pending_todos: i64,
  pub completed_todos: i64,
  pub high_priority_todos: i64,
  pub overdue_todos: i64,
  pub completion_rate: f64
}

impl Priority {
  pub fn from_str(source: &str) -> Result<Self> {
    match source.to_lowercase().as_str() {
      "low" | "낮음" | "l" => Ok(Priority::Low),
      "medium" | "보통" | "m" => Ok(Priority::Medium),
      "high" | "높음" | "h" => Ok(Priority::High),
      _ => Err(TaskyError::InvalidPriority { priority: source.to_string(), }),
    }
  }

  pub fn to_display_string(&self) -> &'static str {
    match self {
        Priority::Low => "낮음",
        Priority::Medium => "보통",
        Priority::High => "높음",
    }
  }

  pub fn to_emoji(&self) -> &'static str {
      match self {
          Priority::Low => "🟢",
          Priority::Medium => "🟡",
          Priority::High => "🔴",
      }
  }
}

impl fmt::Display for Priority {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.to_display_string())
  }
}

impl Default for Priority {
  fn default() -> Self {
    Priority::Medium
  }
}

impl Status {
  pub fn from_str(source: &str) -> Result<Self> {
    match source.to_lowercase().as_str() {
      "pending" | "대기" | "p" => Ok(Status::Pending),
      "done" | "완료" | "d" => Ok(Status::Done),
      _ => Err(TaskyError::InvalidStatus { status: source.to_string(), }),
    }
  }

  pub fn to_display_string(&self) -> &'static str {
    match self {
      Status::Pending => "대기중",
      Status::Done => "완료",
    }
  }

  pub fn to_emoji(&self) -> &'static str {
    match self {
      Status::Pending => "⏳",
      Status::Done => "✅",
    }
  }
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.to_display_string())
  }
}

impl Default for Status {
  fn default() -> Self {
      Status::Pending
  }
}

impl SortBy {
  pub fn from_str(source: &str) -> Result<Self> {
    match source.to_lowercase().as_str() {
        "created" | "created_at" => Ok(SortBy::CreatedAt),
        "updated" | "updated_at" => Ok(SortBy::UpdatedAt),
        "due" | "due_date" => Ok(SortBy::DueDate),
        "priority" => Ok(SortBy::Priority),
        "title" => Ok(SortBy::Title),
        _ => Err(TaskyError::InvalidSortBy { sort_by: source.to_string(), }),
    }
  }
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::CreatedAt
    }
}

impl SortOrder {
    pub fn from_str(source: &str) -> Result<Self> {
      match source.to_lowercase().as_str() {
          "asc" | "ascending" => Ok(SortOrder::Asc),
          "desc" | "descending" => Ok(SortOrder::Desc),
          _ => Ok(SortOrder::Desc),
      }
    }
}

impl Default for SortOrder {
  fn default() -> Self {
      SortOrder::Desc
  }
}

impl CreateTodo {
  pub fn new(title: String) -> Self {
    Self {
      title,
      description: None,
      priority: Priority::default(),
      due_date: None
    }
  }

  pub fn with_description(mut self, description: String) -> Self {
    self.description = Some(description);
    self
  }

  pub fn with_priority(mut self, priority: Priority) -> Self {
    self.priority = priority;
    self
  }

  pub fn with_due_date (mut self, due_date: DateTime<Utc>) -> Self {
    self.due_date = Some(due_date);
    self
  }
}

impl Todo {
  pub fn is_overdue(&self) -> bool {
    if self.status == Status::Done {
      return false;
    }

    if let Some(due_date) = self.due_date {
      due_date < Utc::now()
    } else {
      false
    }
  }

  pub fn days_until_due(&self) -> Option<i64> {
    self.due_date.map(|due: DateTime<Utc>| {
      let now = Utc::now();
      (due - now).num_days()
    })
  }
}