# 문제 해결 가이드

Tasky 사용 중 발생할 수 있는 문제들과 해결 방법을 정리했습니다.

## 🚀 설치 및 초기 설정 문제

### Rust 컴파일러가 없다는 오류
```
error: could not find `rustc` compiler
```

**해결 방법:**
1. Rust 설치: https://rustup.rs/
2. 환경 변수 확인:
   ```bash
   rustc --version
   cargo --version
   ```

### 컴파일 오류
```
error: failed to compile `tasky`
```

**해결 방법:**
1. Rust 버전 업데이트:
   ```bash
   rustup update
   ```
2. 의존성 캐시 정리:
   ```bash
   cargo clean
   cargo build --release
   ```

### 데이터베이스 초기화 실패
```
오류: Failed to initialize database
```

**해결 방법:**
1. 권한 확인:
   ```bash
   # Linux/macOS
   chmod 755 ~/.local/share/tasky/

   # Windows - 관리자 권한으로 실행
   ```
2. 강제 초기화:
   ```bash
   tasky init --force
   ```

---

## 💾 데이터베이스 문제

### 데이터베이스 파일을 찾을 수 없음
```
오류: Database file not found
```

**해결 방법:**
1. 데이터베이스 재초기화:
   ```bash
   tasky init
   ```
2. 데이터베이스 경로 확인:
   - Linux/macOS: `~/.local/share/tasky/tasky.db`
   - Windows: `%APPDATA%\tasky\tasky.db`

### 데이터베이스 락 오류
```
오류: Database is locked
```

**해결 방법:**
1. 다른 Tasky 프로세스 종료:
   ```bash
   # Linux/macOS
   pkill tasky

   # Windows
   taskkill /f /im tasky.exe
   ```
2. 데이터베이스 파일 권한 확인:
   ```bash
   ls -la ~/.local/share/tasky/tasky.db
   ```

### 데이터베이스 손상
```
오류: Database corruption detected
```

**해결 방법:**
1. 백업이 있는 경우 복원
2. 데이터베이스 재생성 (데이터 손실 발생):
   ```bash
   # 기존 데이터베이스 백업
   cp ~/.local/share/tasky/tasky.db ~/.local/share/tasky/tasky.db.backup

   # 새 데이터베이스 생성
   tasky init --force
   ```

---

## 📝 명령어 실행 문제

### 잘못된 ID 오류
```
오류: Todo with ID 999 not found
```

**해결 방법:**
1. 존재하는 ID 확인:
   ```bash
   tasky list
   ```
2. ID는 1부터 시작하는 양의 정수여야 함

### 잘못된 우선순위 값
```
오류: Invalid priority: 'urgent'
```

**사용 가능한 값:**
- `low`, `낮음`, `l`
- `medium`, `보통`, `m`
- `high`, `높음`, `h`

### 잘못된 날짜 형식
```
오류: Invalid date format
```

**지원하는 날짜 형식:**
- `2024-12-31` (YYYY-MM-DD)
- `2024/12/31`
- `12/31/2024`
- `31/12/2024`
- `Dec 31, 2024`
- `31 Dec 2024`

**해결 방법:**
```bash
# ✅ 올바른 예
tasky add "할일" --due "2024-12-31"
tasky add "할일" --due "Dec 31, 2024"

# ❌ 잘못된 예
tasky add "할일" --due "tomorrow"
tasky add "할일" --due "31-12-2024"
```

### 잘못된 상태 값
```
오류: Invalid status: 'completed'
```

**사용 가능한 값:**
- `pending`, `대기`, `p`
- `done`, `완료`, `d`

### 잘못된 정렬 옵션
```
오류: Invalid sort option
```

**사용 가능한 정렬 기준:**
- `created`, `created_at`
- `updated`, `updated_at`
- `due`, `due_date`
- `priority`
- `title`

**사용 가능한 정렬 순서:**
- `asc`, `ascending`
- `desc`, `descending`

---

## 🎨 표시 문제

### 테이블이 깨져서 보임
```
┌────┬────────┬─
│ ID │ 상태   │
```

**원인:** 터미널 창이 너무 좁음

**해결 방법:**
1. 터미널 창 크기 조정
2. 전체 화면으로 터미널 실행
3. 필터를 사용해 표시할 항목 줄이기:
   ```bash
   tasky list -s pending
   ```

### 한글이 깨져서 보임
```
??? ?? ???: ??? ???
```

**해결 방법:**
1. UTF-8 인코딩 설정:
   ```bash
   export LANG=ko_KR.UTF-8
   export LC_ALL=ko_KR.UTF-8
   ```
2. Windows에서는 코드 페이지 변경:
   ```cmd
   chcp 65001
   ```

### 이모지가 표시되지 않음
```
 높음  대신  🔴 높음
```

**해결 방법:**
1. 이모지를 지원하는 터미널 사용 (Windows Terminal, iTerm2 등)
2. 폰트 확인 (Noto Color Emoji, Apple Color Emoji 등)

---

## ⚡ 성능 문제

### 응답 속도가 느림
**원인:** 데이터베이스에 할일이 너무 많음

**해결 방법:**
1. 완료된 할일 정리:
   ```bash
   # 완료된 할일 확인
   tasky list -s done

   # 불필요한 할일 삭제
   tasky remove <ID>
   ```

2. 필터 사용으로 표시할 데이터 줄이기:
   ```bash
   tasky list -s pending  # 대기 중인 것만
   tasky list --today     # 오늘 것만
   ```

### 메모리 사용량이 높음
**해결 방법:**
1. 할일 개수 확인:
   ```bash
   tasky stats
   ```
2. 오래된 완료 할일들 정리

---

## 🔧 고급 문제 해결

### 데이터베이스 백업 및 복원
```bash
# 백업
cp ~/.local/share/tasky/tasky.db ~/backup/tasky_$(date +%Y%m%d).db

# 복원
cp ~/backup/tasky_20241220.db ~/.local/share/tasky/tasky.db
```

### 설정 파일 위치
- **Linux/macOS**: `~/.local/share/tasky/`
- **Windows**: `%APPDATA%\tasky\`

### 로그 파일 확인
현재 버전에서는 로그 파일을 생성하지 않지만, 오류 발생 시 터미널 출력을 확인하세요.

### 디버그 모드로 실행
```bash
RUST_LOG=debug cargo run -- list
```

---

## 📞 추가 도움

### 문제가 계속 발생하는 경우

1. **GitHub Issues**: https://github.com/TechieQuokka/Tasky/issues
   - 버그 리포트 작성
   - 기능 요청

2. **정보 수집**:
   ```bash
   # 시스템 정보
   uname -a
   rustc --version

   # Tasky 버전
   tasky --version

   # 오류 메시지 전체 복사
   ```

3. **재현 단계 작성**:
   - 어떤 명령어를 실행했는지
   - 예상한 결과
   - 실제 발생한 오류

### 임시 해결책

문제가 해결될 때까지 사용할 수 있는 임시 방법들:

1. **데이터 내보내기** (수동):
   ```bash
   tasky list > my_todos.txt
   ```

2. **기본 기능만 사용**:
   ```bash
   # 복잡한 옵션 없이 기본 명령어만
   tasky add "할일"
   tasky list
   tasky done 1
   ```

3. **새 환경에서 테스트**:
   ```bash
   # 새 디렉토리에서 테스트
   mkdir test_tasky
   cd test_tasky
   # 여기서 Tasky 재빌드 및 테스트
   ```

---

## ✅ 문제 예방

### 정기적인 백업
```bash
# 주간 백업 스크립트 예시
#!/bin/bash
backup_dir="$HOME/tasky_backups"
mkdir -p "$backup_dir"
cp ~/.local/share/tasky/tasky.db "$backup_dir/tasky_$(date +%Y%m%d_%H%M%S).db"

# 30일 이상 된 백업 삭제
find "$backup_dir" -name "tasky_*.db" -mtime +30 -delete
```

### 안전한 사용 습관
1. 중요한 데이터 입력 전 백업
2. 대량 삭제 작업 전 확인
3. 정기적인 데이터 정리
4. 명령어 실행 전 옵션 확인

### 업데이트 관리
```bash
# 정기적인 업데이트 확인
git pull origin master
cargo build --release

# 업데이트 전 백업
cp ~/.local/share/tasky/tasky.db ~/.local/share/tasky/tasky.db.pre_update
```