# Tasky - Rust 개인용 할일 관리 앱 아키텍처

## 개요
Tasky는 Rust로 개발되는 개인용 할일 관리 CLI 애플리케이션입니다. SQLite를 데이터베이스로 사용하여 로컬 환경에서 간단하고 효율적인 할일 관리 기능을 제공합니다.

## 시스템 요구사항
- **언어**: Rust (Edition 2021)
- **데이터베이스**: SQLite
- **인터페이스**: CLI (Command Line Interface)
- **사용자**: 단일 사용자 (개인용)

## 아키텍처 개요

### 레이어 구조
```
┌─────────────────────┐
│  Presentation Layer │  <- CLI 인터페이스
├─────────────────────┤
│ Business Logic Layer│  <- 서비스 로직
├─────────────────────┤
│ Data Access Layer   │  <- SQLite 데이터베이스
└─────────────────────┘
```

### 폴더 구조
```
src/
├── main.rs              // 애플리케이션 엔트리포인트
├── lib.rs               // 라이브러리 루트
├── cli/                 // CLI 관련 모듈
│   ├── mod.rs           // CLI 모듈 정의
│   ├── commands.rs      // CLI 명령어 정의
│   └── args.rs          // 명령어 인자 정의
├── models/              // 데이터 모델
│   ├── mod.rs           // 모델 모듈 정의
│   └── todo.rs          // 할일 데이터 구조
├── services/            // 비즈니스 로직
│   ├── mod.rs           // 서비스 모듈 정의
│   └── todo_service.rs  // 할일 관리 서비스
├── database/            // 데이터베이스 관련
│   ├── mod.rs           // DB 모듈 정의
│   ├── connection.rs    // SQLite 연결 관리
│   └── migrations.rs    // 스키마 마이그레이션
└── utils/               // 유틸리티 함수
    ├── mod.rs
    └── date.rs          // 날짜 처리 유틸리티
```

## 주요 컴포넌트

### 1. CLI Layer (`cli/`)
- **역할**: 사용자 인터페이스 및 명령어 처리
- **주요 기능**:
  - 명령어 파싱 (add, list, done, remove 등)
  - 사용자 입력 검증
  - 결과 출력 포맷팅

### 2. Business Logic Layer (`services/`)
- **역할**: 핵심 비즈니스 로직 처리
- **주요 기능**:
  - 할일 생성, 수정, 삭제
  - 할일 상태 관리
  - 데이터 검증 및 변환

### 3. Data Access Layer (`database/`)
- **역할**: SQLite 데이터베이스 연동
- **주요 기능**:
  - 데이터베이스 연결 관리
  - SQL 쿼리 실행
  - 트랜잭션 관리

### 4. Data Models (`models/`)
- **역할**: 데이터 구조 정의
- **주요 구조체**:
  - `Todo`: 할일 정보
  - `Priority`: 우선순위 열거형
  - `Status`: 할일 상태 열거형

## 데이터베이스 스키마

### todos 테이블
```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 0, -- 0: Low, 1: Medium, 2: High
    status INTEGER NOT NULL DEFAULT 0,   -- 0: Pending, 1: Done
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    due_date TEXT
);
```

## 사용할 주요 Crates

### 필수 Dependencies
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
rusqlite = { version = "0.29", features = ["bundled", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
```

### 개발용 Dependencies
```toml
[dev-dependencies]
tempfile = "3.0"
assert_cmd = "2.0"
predicates = "3.0"
```

## CLI 명령어 설계

### 기본 명령어
```bash
# 할일 추가
tasky add "프로젝트 완료" --priority high --due "2024-01-15"

# 할일 목록 보기
tasky list
tasky list --status pending
tasky list --priority high

# 할일 완료 처리
tasky done 1

# 할일 삭제
tasky remove 1

# 할일 수정
tasky edit 1 --title "새로운 제목"
```

### 추가 기능
```bash
# 통계 보기
tasky stats

# 데이터베이스 초기화
tasky init

# 백업/복원
tasky backup
tasky restore backup.db
```

## 에러 처리 전략

### 에러 타입 정의
```rust
#[derive(thiserror::Error, Debug)]
pub enum TaskyError {
    #[error("데이터베이스 오류: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("할일을 찾을 수 없습니다: ID {id}")]
    TodoNotFound { id: i64 },

    #[error("잘못된 입력: {message}")]
    InvalidInput { message: String },

    #[error("시스템 오류: {0}")]
    System(#[from] anyhow::Error),
}
```

## 테스트 전략

### 단위 테스트
- 각 모듈별 독립적인 테스트
- Mock을 활용한 데이터베이스 테스트

### 통합 테스트
- CLI 명령어 전체 플로우 테스트
- 임시 데이터베이스를 활용한 E2E 테스트

### 테스트 구조
```
tests/
├── integration/
│   ├── cli_tests.rs
│   └── database_tests.rs
└── common/
    └── mod.rs
```

## 성능 최적화 고려사항

1. **데이터베이스 최적화**
   - 적절한 인덱스 생성
   - 연결 풀링 (필요시)
   - 트랜잭션 최적화

2. **메모리 관리**
   - 불필요한 복사 최소화
   - 스택 할당 우선 사용
   - Iterator 활용

3. **I/O 최적화**
   - 배치 처리
   - 버퍼링 활용

## 보안 고려사항

1. **SQL 인젝션 방지**
   - Prepared Statement 사용
   - 입력 검증

2. **파일 권한**
   - 데이터베이스 파일 접근 권한 제한
   - 백업 파일 보안

## 확장성 고려사항

### 미래 확장 가능 기능
1. **다중 프로젝트 지원**
2. **태그 시스템**
3. **반복 작업**
4. **알림 기능**
5. **웹 인터페이스**
6. **동기화 기능**

### 아키텍처 확장성
- 인터페이스 기반 설계로 구현체 교체 가능
- 플러그인 시스템 고려
- 설정 파일 지원

이 아키텍처는 단순함과 확장성의 균형을 맞춰 개인 프로젝트로 시작하되, 필요시 기능을 추가할 수 있도록 설계되었습니다.