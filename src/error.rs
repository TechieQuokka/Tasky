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

    #[error("날짜 파싱 오류: {0}")]
    ChronoParse(#[from] chrono::ParseError),
}

pub type Result<T> = std::result::Result<T, TaskyError>;