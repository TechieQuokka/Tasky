# 기여하기

Tasky 프로젝트에 기여해주셔서 감사합니다! 이 문서는 프로젝트에 기여하는 방법을 안내합니다.

## 🎯 기여 방법

### 1. 버그 리포트
- 명확한 재현 단계 제공
- 예상 결과와 실제 결과 설명
- 환경 정보 포함 (OS, Rust 버전 등)

### 2. 기능 제안
- 기능의 필요성과 사용 사례 설명
- 가능한 구현 방법 제시
- 기존 기능과의 호환성 고려

### 3. 코드 기여
- 버그 수정
- 새 기능 구현
- 성능 개선
- 문서 개선

### 4. 문서화
- 사용법 가이드 개선
- 코드 주석 추가
- 예제 코드 작성
- 번역 작업

---

## 🚀 시작하기

### 1. 개발 환경 설정

```bash
# 저장소 포크 및 클론
git clone https://github.com/YOUR_USERNAME/Tasky.git
cd Tasky

# 업스트림 저장소 추가
git remote add upstream https://github.com/TechieQuokka/Tasky.git

# 의존성 설치 및 빌드
cargo build

# 테스트 실행
cargo test
```

### 2. 개발 도구 설치

```bash
# 코드 포맷터
rustup component add rustfmt

# 린터
rustup component add clippy

# 문서 생성
cargo install cargo-doc
```

---

## 📝 개발 가이드라인

### 코드 스타일

1. **Rust 표준 스타일 준수**
   ```bash
   # 자동 포맷팅
   cargo fmt

   # 린트 검사
   cargo clippy
   ```

2. **변수 및 함수 명명**
   - `snake_case` 사용
   - 명확하고 설명적인 이름
   - 한글 주석 권장

   ```rust
   // ✅ 좋은 예
   fn create_todo_with_priority(title: String, priority: Priority) -> Result<Todo> {
       // 할일을 생성하고 우선순위를 설정
   }

   // ❌ 나쁜 예
   fn createTodo(t: String, p: Priority) -> Result<Todo> {
   }
   ```

3. **오류 처리**
   - `Result<T, E>` 타입 적극 활용
   - 사용자 친화적 오류 메시지
   - 체인 가능한 오류 타입

   ```rust
   // ✅ 좋은 예
   pub fn parse_date(input: &str) -> Result<DateTime<Utc>> {
       // 다양한 날짜 형식 시도
       for format in DATE_FORMATS {
           if let Ok(date) = NaiveDate::parse_from_str(input, format) {
               return Ok(date.and_hms(0, 0, 0).and_utc());
           }
       }
       Err(TaskyError::InvalidDateFormat {
           input: input.to_string(),
           supported_formats: DATE_FORMATS.to_vec(),
       })
   }
   ```

### 커밋 메시지

```
<타입>: <간단한 설명>

<상세 설명>

관련 이슈: #123
```

**타입:**
- `feat`: 새 기능
- `fix`: 버그 수정
- `docs`: 문서 변경
- `style`: 코드 포맷팅
- `refactor`: 리팩토링
- `test`: 테스트 추가/수정
- `chore`: 빌드/설정 변경

**예시:**
```
feat: 할일 검색 기능 추가

제목과 설명 내용으로 할일을 검색할 수 있는 기능을 추가했습니다.
- 대소문자 구분 없는 검색
- 정규표현식 지원
- 부분 문자열 매칭

관련 이슈: #42
```

---

## 🧪 테스트

### 테스트 작성 가이드

1. **단위 테스트**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_create_todo_with_valid_title() {
           let todo = CreateTodo::new("테스트 할일".to_string());
           assert_eq!(todo.title, "테스트 할일");
           assert_eq!(todo.priority, Priority::Medium);
       }

       #[test]
       fn test_priority_from_korean_string() {
           assert_eq!(Priority::from_str("높음").unwrap(), Priority::High);
           assert_eq!(Priority::from_str("보통").unwrap(), Priority::Medium);
           assert_eq!(Priority::from_str("낮음").unwrap(), Priority::Low);
       }
   }
   ```

2. **통합 테스트**
   ```rust
   // tests/integration_test.rs
   use tasky::cli::execute;
   use tasky::cli::args::Commands;

   #[test]
   fn test_add_and_list_todo() {
       // 임시 데이터베이스 생성
       let temp_dir = tempfile::tempdir().unwrap();
       let db_path = temp_dir.path().join("test.db");

       // 할일 추가 테스트
       let add_command = Commands::Add {
           title: "테스트 할일".to_string(),
           description: None,
           priority: "medium".to_string(),
           due: None,
       };

       execute(add_command).unwrap();
       // 추가 검증 로직...
   }
   ```

### 테스트 실행

```bash
# 모든 테스트 실행
cargo test

# 특정 테스트만 실행
cargo test test_create_todo

# 통합 테스트만 실행
cargo test --test integration_test

# 테스트 커버리지 확인 (tarpaulin 설치 필요)
cargo tarpaulin --out Html
```

---

## 📖 문서화

### 코드 문서화

```rust
/// 새로운 할일을 생성합니다.
///
/// # 인자
///
/// * `title` - 할일의 제목
/// * `description` - 할일의 상세 설명 (선택사항)
/// * `priority` - 우선순위 (기본값: Medium)
///
/// # 예시
///
/// ```
/// use tasky::models::{CreateTodo, Priority};
///
/// let todo = CreateTodo::new("장보기".to_string())
///     .with_description("우유, 빵, 계란 구매".to_string())
///     .with_priority(Priority::High);
/// ```
///
/// # 오류
///
/// 제목이 비어있거나 255자를 초과하는 경우 `TaskyError::InvalidTitle`을 반환합니다.
pub fn create_todo(todo: CreateTodo) -> Result<Todo> {
    // 구현...
}
```

### 문서 생성

```bash
# API 문서 생성
cargo doc --open

# 모든 의존성 포함하여 문서 생성
cargo doc --document-private-items --open
```

---

## 🔍 코드 리뷰

### 리뷰 체크리스트

**기능적 측면:**
- [ ] 기능이 요구사항을 충족하는가?
- [ ] 엣지 케이스가 처리되는가?
- [ ] 오류 처리가 적절한가?
- [ ] 성능상 문제가 없는가?

**코드 품질:**
- [ ] 코드가 읽기 쉬운가?
- [ ] 함수/모듈이 적절히 분리되어 있는가?
- [ ] 네이밍이 명확한가?
- [ ] 불필요한 복잡성이 없는가?

**테스트:**
- [ ] 적절한 테스트가 작성되었는가?
- [ ] 모든 테스트가 통과하는가?
- [ ] 테스트 커버리지가 충분한가?

**문서화:**
- [ ] 공개 API에 문서가 있는가?
- [ ] 복잡한 로직에 주석이 있는가?
- [ ] README가 업데이트되었는가?

### Pull Request 가이드라인

1. **제목과 설명**
   ```
   feat: 할일 검색 기능 구현

   ## 변경사항
   - 제목/설명으로 할일 검색 가능
   - 대소문자 구분 없는 검색
   - 정규표현식 지원

   ## 테스트
   - 단위 테스트 추가
   - 통합 테스트 추가
   - 수동 테스트 완료

   ## 관련 이슈
   Closes #42
   ```

2. **체크리스트**
   - [ ] 모든 테스트 통과
   - [ ] 코드 포맷팅 완료 (`cargo fmt`)
   - [ ] 린트 검사 통과 (`cargo clippy`)
   - [ ] 문서화 완료
   - [ ] CHANGELOG 업데이트

3. **리뷰 요청**
   - 명확한 변경 사항 설명
   - 스크린샷/예시 제공
   - 특별히 검토받고 싶은 부분 명시

---

## 🚢 릴리스 프로세스

### 버전 관리

Semantic Versioning (SemVer) 사용:
- `MAJOR.MINOR.PATCH` (예: 1.2.3)
- **MAJOR**: 호환성을 깨는 변경
- **MINOR**: 새 기능 추가 (하위 호환)
- **PATCH**: 버그 수정

### 릴리스 체크리스트

1. **준비 단계**
   - [ ] 모든 테스트 통과
   - [ ] 문서 업데이트
   - [ ] CHANGELOG.md 업데이트
   - [ ] Cargo.toml 버전 업데이트

2. **릴리스 단계**
   - [ ] Git 태그 생성
   - [ ] GitHub Release 생성
   - [ ] 바이너리 빌드 및 업로드
   - [ ] 문서 사이트 업데이트

---

## 🎨 UI/UX 가이드라인

### 명령줄 인터페이스

1. **사용자 친화적 메시지**
   ```rust
   // ✅ 좋은 예
   println!("{} 할일이 추가되었습니다!", "✅".green());
   println!("  ID: {}", todo.id.unwrap_or(0).to_string().cyan());

   // ❌ 나쁜 예
   println!("Todo created with ID {}", todo.id.unwrap_or(0));
   ```

2. **오류 메시지**
   ```rust
   // ✅ 좋은 예
   eprintln!("{} 할일을 찾을 수 없습니다. ID {}는 존재하지 않습니다.",
            "❌".red(), id);
   eprintln!("사용 가능한 할일 목록을 보려면 'tasky list'를 실행하세요.");

   // ❌ 나쁜 예
   eprintln!("Todo not found: {}", id);
   ```

3. **일관된 색상 사용**
   - 🟢 **녹색**: 성공, 완료, 낮은 우선순위
   - 🟡 **노란색**: 경고, 대기, 보통 우선순위
   - 🔴 **빨간색**: 오류, 높은 우선순위, 기한 초과
   - 🔵 **파란색**: 정보, 수정
   - **회색**: 비활성, 삭제됨

---

## 🔧 개발 도구

### 권장 에디터 설정

**VSCode (.vscode/settings.json):**
```json
{
    "rust-analyzer.cargo.features": ["all"],
    "rust-analyzer.checkOnSave.command": "clippy",
    "editor.formatOnSave": true,
    "files.insertFinalNewline": true,
    "files.trimTrailingWhitespace": true
}
```

**VSCode 확장:**
- rust-analyzer
- Error Lens
- Better TOML
- GitLens

### 개발 스크립트

```bash
#!/bin/bash
# scripts/dev-check.sh

echo "🔍 코드 포맷팅 검사..."
cargo fmt --check

echo "🔍 린트 검사..."
cargo clippy -- -D warnings

echo "🧪 테스트 실행..."
cargo test

echo "📖 문서 검사..."
cargo doc --no-deps

echo "✅ 모든 검사 완료!"
```

---

## 📞 소통 및 지원

### GitHub Issues

**버그 리포트 템플릿:**
```markdown
## 버그 설명
버그에 대한 명확한 설명

## 재현 단계
1. '...' 명령어 실행
2. '...' 입력
3. 오류 발생

## 예상 동작
정상적으로 동작해야 하는 내용

## 실제 동작
실제로 발생한 동작

## 환경 정보
- OS: [예: Ubuntu 20.04]
- Rust 버전: [예: 1.75.0]
- Tasky 버전: [예: 0.1.0]

## 추가 컨텍스트
스크린샷, 로그 등 추가 정보
```

**기능 요청 템플릿:**
```markdown
## 기능 설명
원하는 기능에 대한 명확한 설명

## 문제점
현재 어떤 문제나 불편함이 있는지

## 제안 해결책
어떻게 해결하면 좋을지 제안

## 대안
고려해볼 수 있는 다른 방법들

## 추가 컨텍스트
관련 이미지, 링크, 참고 자료 등
```

### 코드 리뷰 예절

1. **건설적인 피드백**
   - 문제점과 함께 개선 방안 제시
   - 개인이 아닌 코드에 대한 리뷰
   - 긍정적인 부분도 언급

2. **명확한 의사소통**
   - 구체적인 예시 제공
   - 우선순위 표시 (필수/제안)
   - 질문과 의견 구분

---

## 🏆 기여자 인정

### 기여 유형

- **Code**: 코드 기여
- **Doc**: 문서 기여
- **Bug**: 버그 발견 및 리포트
- **Ideas**: 아이디어 제안
- **Review**: 코드 리뷰
- **Test**: 테스트 작성
- **Translation**: 번역 작업

### 기여자 목록

README.md에 기여자를 추가하여 감사를 표합니다.

---

Tasky 프로젝트에 기여해주셔서 다시 한 번 감사합니다! 궁금한 점이 있으시면 언제든 Issue를 생성하거나 토론에 참여해주세요.