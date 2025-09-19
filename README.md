# Tasky

## 개요
Tasky는 Rust로 개발된 명령줄 인터페이스(CLI) 기반의 작업 관리 도구입니다.

## 기능
- 작업 생성, 조회, 수정, 삭제
- **작업 설명(description) 추가 및 상세 보기**
- 우선순위 설정 (low, medium, high)
- 마감일 관리 (다양한 날짜 형식 지원: YYYY-MM-DD, YYYY.MM.DD, YYYY/MM/DD, +n/-n)
- **🆕 상대 날짜 입력 지원 (+n/-n 형식)** - v0.1.5
- **🆕 터미널 크기 자동 적응 및 유니코드 지원** - v0.1.5
- **🆕 개선된 마감일 표시 ("n일 전/후" 형태)** - v0.1.5
- 상태 추적 (pending, done)
- SQLite 데이터베이스를 통한 데이터 저장
- 통계 및 진행률 확인
- 견고한 데이터베이스 초기화 및 복구

## 빠른 시작

### 1. 설치

#### 방법 1: 컴파일된 실행파일 다운로드 (추천)

**📋 빠른 설치를 원한다면**: [quick-start.md](docs/quick-start.md) 가이드를 참조하세요!

[Releases 페이지](https://github.com/TechieQuokka/Tasky/releases)에서 운영체체에 맞는 실행파일을 다운로드하세요:

- **Windows (x64)**: `tasky-windows-x64.exe`
- **Linux (x64)**: `tasky-linux-x64`
- **macOS (Intel)**: `tasky-macos-x64`
- **macOS (Apple Silicon)**: `tasky-macos-arm64`

**Windows 설치 (PowerShell):**
```powershell
# 1. 설치 경로 설정 (원하는 경로로 변경하세요!)
$installPath = "C:\tools\tasky"  # 👈 여기를 원하는 경로로 변경
$downloadFile = "$env:USERPROFILE\Downloads\tasky-windows-x64.exe"  # 👈 실제 다운로드 위치

# 2. 폴더 생성 및 파일 이동
New-Item -ItemType Directory -Path $installPath -Force
Move-Item $downloadFile "$installPath\tasky.exe"

# 3. PATH 환경변수 설정
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$newPath = $currentPath + ";" + $installPath
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

# 4. 새 PowerShell 창에서 테스트
# Write-Host "새 PowerShell 창을 열고 다음 명령어를 실행하세요:"
# tasky --help
```

> ⚠️ **중요**: 위 스크립트의 `$installPath`와 `$downloadFile` 경로를 실제 환경에 맞게 수정하세요!
>
> 자세한 설명은 [docs/quick-start.md](docs/quick-start.md)를 참조하세요.

**Linux/macOS 설치:**
```bash
# 다운로드 (Linux 예시)
wget https://github.com/TechieQuokka/Tasky/releases/latest/download/tasky-linux-x64

# 실행 권한 부여
chmod +x tasky-linux-x64

# /usr/local/bin으로 이동 (권장)
sudo mv tasky-linux-x64 /usr/local/bin/tasky

# 테스트
tasky --help
```

#### 방법 2: 소스코드에서 빌드
```bash
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky
cargo build --release
```

### 2. 데이터베이스 초기화
```bash
# 최초 실행 시 데이터베이스 초기화
cargo run -- init

# 기존 데이터베이스 재초기화 (강제)
cargo run -- init --force
```

### 3. 기본 사용법
```bash
# 작업 추가 (설명 포함)
cargo run -- add "새로운 작업" -d "작업에 대한 상세 설명" --priority high --due 2024-12-31

# 🆕 상대 날짜로 작업 추가 (v0.1.5+)
cargo run -- add "내일 미팅" --due "+1" --priority high    # 1일 후
cargo run -- add "3일 후 프레젠테이션" --due "+3"           # 3일 후
cargo run -- add "어제 놓친 일정" --due "-1"                # 1일 전 (어제)

# 작업 목록 조회
cargo run -- list

# 작업 상세 정보 보기
cargo run -- show <ID>

# 설명 포함 상세 목록 보기
cargo run -- list --verbose
cargo run -- list -v

# 특정 조건으로 필터링
cargo run -- list --status pending --priority high
cargo run -- list --today           # 오늘 마감인 작업
cargo run -- list --overdue         # 기한 초과 작업
cargo run -- list --urgent          # 긴급 작업

# 작업 완료 처리
cargo run -- done <ID>

# 작업 상태 되돌리기
cargo run -- undone <ID>

# 작업 수정 (제목, 설명, 우선순위 등)
cargo run -- edit <ID> --title "수정된 제목" --description "새로운 설명" --priority medium

# 작업 삭제
cargo run -- remove <ID>

# 통계 확인
cargo run -- stats

# 데이터베이스 정보 확인
cargo run -- db-info
```

## 환경변수 설정

### Windows (PowerShell)
```powershell
# 임시 설정 (현재 세션만)
$env:TASKY_DB_PATH = "C:\MyData\tasky.db"

# 영구 설정 (사용자 환경변수)
[Environment]::SetEnvironmentVariable("TASKY_DB_PATH", "C:\MyData\tasky.db", "User")

# 영구 설정 (시스템 환경변수) - 관리자 권한 필요
[Environment]::SetEnvironmentVariable("TASKY_DB_PATH", "C:\MyData\tasky.db", "Machine")
```

### Windows (명령 프롬프트)
```cmd
# 임시 설정 (현재 세션만)
set TASKY_DB_PATH=C:\MyData\tasky.db

# 영구 설정 (사용자 환경변수)
setx TASKY_DB_PATH "C:\MyData\tasky.db"

# 영구 설정 (시스템 환경변수) - 관리자 권한 필요
setx TASKY_DB_PATH "C:\MyData\tasky.db" /M
```

### Linux/macOS (Bash/Zsh)
```bash
# 임시 설정 (현재 세션만)
export TASKY_DB_PATH="/home/user/data/tasky.db"

# 영구 설정 (.bashrc, .zshrc, .profile 등에 추가)
echo 'export TASKY_DB_PATH="/home/user/data/tasky.db"' >> ~/.bashrc
source ~/.bashrc

# 또는 .profile에 추가
echo 'export TASKY_DB_PATH="/home/user/data/tasky.db"' >> ~/.profile
```

### 지원하는 환경변수
| 변수명 | 설명 | 기본값 |
|--------|------|--------|
| `TASKY_DB_PATH` | 데이터베이스 파일 경로 | Windows: `%APPDATA%\tasky\tasky.db`<br>Linux/macOS: `~/.local/share/tasky/tasky.db` |

### 환경변수 확인
```bash
# 현재 설정된 환경변수 확인
cargo run -- db-info
```

## 트러블슈팅

### 데이터베이스 문제

#### "다른 프로세스가 파일을 사용 중" 오류
```bash
오류: I/O 오류: 다른 프로세스가 파일을 사용 중이기 때문에 프로세스가 액세스 할 수 없습니다. (os error 32)
```

**해결 방법:**
1. **다른 Tasky 프로세스 종료**
   ```bash
   # Windows 작업 관리자에서 tasky.exe 프로세스 종료
   # 또는 명령 프롬프트에서:
   taskkill /f /im tasky.exe
   ```

2. **SQLite 브라우저나 DB 도구 종료**
   - DB Browser for SQLite, DBeaver 등이 데이터베이스 파일을 열고 있는지 확인
   - 해당 프로그램들을 모두 종료

3. **바이러스 백신 스캔 대기**
   - 바이러스 백신이 파일을 스캔 중일 수 있음
   - 잠시 기다린 후 다시 시도

4. **강제 초기화 사용**
   ```bash
   cargo run -- init --force
   ```
   - 자동으로 여러 해결 전략을 시도합니다

#### 데이터베이스 파일 손상
```bash
# 데이터베이스 상태 확인
cargo run -- db-info

# 손상된 경우 강제 재초기화
cargo run -- init --force
```

#### 권한 문제
**Windows:**
```powershell
# 관리자 권한으로 PowerShell 실행 후:
icacls "C:\Users\사용자명\AppData\Roaming\tasky" /grant 사용자명:F /T
```

**Linux/macOS:**
```bash
# 데이터베이스 디렉토리 권한 확인
ls -la ~/.local/share/tasky/

# 권한 수정
chmod 755 ~/.local/share/tasky/
chmod 644 ~/.local/share/tasky/tasky.db
```

### 빌드 문제

#### Rust 버전 호환성
```bash
# Rust 버전 확인 (최소 1.70.0 권장)
rustc --version

# Rust 업데이트
rustup update stable
```

#### 의존성 문제
```bash
# 의존성 캐시 정리
cargo clean

# 의존성 재설치
cargo build --release
```

### 성능 문제

#### 대용량 데이터베이스
```bash
# 데이터베이스 크기 확인
cargo run -- db-info

# 완료된 작업 정리 (수동으로 구현 예정)
# SQLite 데이터베이스 최적화
sqlite3 경로/tasky.db "VACUUM;"
```

### 일반적인 문제

#### 명령어를 찾을 수 없음
```bash
# PATH에 추가하거나 절대 경로 사용
./target/release/tasky <명령어>

# 또는 cargo run 사용
cargo run -- <명령어>
```

#### 도움말 확인
```bash
# 전체 도움말
cargo run -- --help

# 특정 명령어 도움말
cargo run -- add --help
cargo run -- list --help
```

## 기술 스택
- **언어**: Rust (최소 1.70.0)
- **데이터베이스**: SQLite 3
- **CLI 프레임워크**: clap 4.x
- **날짜/시간**: chrono
- **컬러 출력**: colored
- **테이블 출력**: prettytable-rs
- **터미널 크기 감지**: terminal_size - v0.1.5+
- **유니코드 지원**: unicode-width - v0.1.5+
- **에러 처리**: thiserror, anyhow

## 프로젝트 구조
```
src/
├── cli/           # CLI 명령어 처리
├── models/        # 데이터 모델
├── utils/         # 유틸리티 함수
│   ├── date.rs    # 날짜 파싱 및 처리
│   └── text.rs    # 텍스트 처리 및 터미널 적응 (v0.1.5+)
└── main.rs        # 메인 진입점
```

## 라이선스
MIT License

## 기여하기
1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request