# 데이터 모델 및 인터페이스 설계

## 데이터 모델

### Todo 구조체
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending = 0,
    Done = 1,
}
```

### 새 할일 생성용 구조체
```rust
#[derive(Debug, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub due_date: Option<DateTime<Utc>>,
}
```

### 검색/필터링용 구조체
```rust
#[derive(Debug, Clone, Default)]
pub struct TodoFilter {
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub created_before: Option<DateTime<Utc>>,
    pub created_after: Option<DateTime<Utc>>,
    pub due_before: Option<DateTime<Utc>>,
    pub due_after: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum SortBy {
    CreatedAt,
    UpdatedAt,
    DueDate,
    Priority,
    Title,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}
```

## 서비스 인터페이스

### TodoService Trait
```rust
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TodoService: Send + Sync {
    /// 새 할일 생성
    async fn create_todo(&self, create_todo: CreateTodo) -> Result<Todo>;

    /// ID로 할일 조회
    async fn get_todo_by_id(&self, id: i64) -> Result<Option<Todo>>;

    /// 모든 할일 조회 (필터링 및 정렬 포함)
    async fn list_todos(
        &self,
        filter: Option<TodoFilter>,
        sort_by: Option<SortBy>,
        sort_order: Option<SortOrder>,
    ) -> Result<Vec<Todo>>;

    /// 할일 수정
    async fn update_todo(&self, id: i64, update_todo: UpdateTodo) -> Result<Option<Todo>>;

    /// 할일 삭제
    async fn delete_todo(&self, id: i64) -> Result<bool>;

    /// 할일 완료 처리
    async fn complete_todo(&self, id: i64) -> Result<bool>;

    /// 할일 통계 조회
    async fn get_stats(&self) -> Result<TodoStats>;
}
```

### 데이터베이스 리포지토리 인터페이스
```rust
#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, todo: &CreateTodo) -> Result<Todo>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Todo>>;
    async fn find_all(&self, filter: &TodoFilter, sort_by: SortBy, sort_order: SortOrder) -> Result<Vec<Todo>>;
    async fn update(&self, id: i64, todo: &UpdateTodo) -> Result<Option<Todo>>;
    async fn delete(&self, id: i64) -> Result<bool>;
    async fn count_by_status(&self, status: Status) -> Result<i64>;
}
```

## 통계 데이터 모델
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoStats {
    pub total_todos: i64,
    pub pending_todos: i64,
    pub completed_todos: i64,
    pub high_priority_todos: i64,
    pub overdue_todos: i64,
    pub completion_rate: f64,
}
```

## CLI 명령어 모델

### CLI 인자 구조체
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tasky")]
#[command(about = "개인용 할일 관리 도구")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 새 할일 추가
    Add {
        /// 할일 제목
        title: String,
        /// 할일 설명
        #[arg(short, long)]
        description: Option<String>,
        /// 우선순위 (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        /// 마감일 (YYYY-MM-DD 형식)
        #[arg(short, long)]
        due: Option<String>,
    },

    /// 할일 목록 조회
    List {
        /// 상태 필터 (pending, done)
        #[arg(short, long)]
        status: Option<String>,
        /// 우선순위 필터 (low, medium, high)
        #[arg(short, long)]
        priority: Option<String>,
        /// 정렬 기준 (created, updated, due, priority, title)
        #[arg(long, default_value = "created")]
        sort: String,
        /// 정렬 순서 (asc, desc)
        #[arg(long, default_value = "desc")]
        order: String,
    },

    /// 할일 완료 처리
    Done {
        /// 할일 ID
        id: i64,
    },

    /// 할일 삭제
    Remove {
        /// 할일 ID
        id: i64,
    },

    /// 할일 수정
    Edit {
        /// 할일 ID
        id: i64,
        /// 새 제목
        #[arg(short, long)]
        title: Option<String>,
        /// 새 설명
        #[arg(short, long)]
        description: Option<String>,
        /// 새 우선순위
        #[arg(short, long)]
        priority: Option<String>,
        /// 새 마감일
        #[arg(long)]
        due: Option<String>,
    },

    /// 통계 조회
    Stats,

    /// 데이터베이스 초기화
    Init,
}
```

## 에러 타입 모델
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskyError {
    #[error("데이터베이스 오류: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("할일을 찾을 수 없습니다 (ID: {id})")]
    TodoNotFound { id: i64 },

    #[error("잘못된 우선순위: {priority}. low, medium, high 중 하나여야 합니다")]
    InvalidPriority { priority: String },

    #[error("잘못된 상태: {status}. pending, done 중 하나여야 합니다")]
    InvalidStatus { status: String },

    #[error("잘못된 날짜 형식: {date}. YYYY-MM-DD 형식이어야 합니다")]
    InvalidDateFormat { date: String },

    #[error("잘못된 정렬 기준: {sort_by}")]
    InvalidSortBy { sort_by: String },

    #[error("할일 제목은 비어있을 수 없습니다")]
    EmptyTitle,

    #[error("시스템 오류: {0}")]
    System(#[from] anyhow::Error),

    #[error("I/O 오류: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TaskyError>;
```

## 구현체 예시

### Priority와 Status 구현
```rust
impl Priority {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" | "낮음" => Ok(Priority::Low),
            "medium" | "보통" => Ok(Priority::Medium),
            "high" | "높음" => Ok(Priority::High),
            _ => Err(TaskyError::InvalidPriority { priority: s.to_string() }),
        }
    }

    pub fn to_display_string(&self) -> &'static str {
        match self {
            Priority::Low => "낮음",
            Priority::Medium => "보통",
            Priority::High => "높음",
        }
    }
}

impl Status {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pending" | "대기" => Ok(Status::Pending),
            "done" | "완료" => Ok(Status::Done),
            _ => Err(TaskyError::InvalidStatus { status: s.to_string() }),
        }
    }

    pub fn to_display_string(&self) -> &'static str {
        match self {
            Status::Pending => "대기중",
            Status::Done => "완료",
        }
    }
}
```

### Default 구현
```rust
impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

impl Default for CreateTodo {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            priority: Priority::default(),
            due_date: None,
        }
    }
}
```

이 데이터 모델 설계는 타입 안전성을 보장하면서도 확장 가능한 구조를 제공합니다. Rust의 특성을 잘 활용하여 컴파일 타임에 많은 오류를 잡을 수 있도록 설계되었습니다.