# Tasky 빠른 시작 가이드

Tasky는 명령줄에서 사용하는 간단하고 효율적인 할일 관리 도구입니다. 몇 분 안에 설치하고 바로 사용할 수 있습니다.

## 1. 다운로드

[GitHub Releases](https://github.com/TechieQuokka/Tasky/releases/latest)에서 운영체제에 맞는 실행 파일을 다운로드하세요:

- **Windows**: `tasky-windows-x64.exe`
- **Linux**: `tasky-linux-x64`
- **macOS (Intel)**: `tasky-macos-x64`
- **macOS (Apple Silicon)**: `tasky-macos-arm64`

## 2. 설치

### Windows

```powershell
# 설치할 폴더 경로 (원하는 경로로 변경 가능)
$installPath = "C:\tools\tasky"

# 폴더 생성
New-Item -ItemType Directory -Path $installPath -Force

# 다운로드 파일을 설치 폴더로 이동 (다운로드 경로 확인 후 수정)
Move-Item "$env:USERPROFILE\Downloads\tasky-windows-x64.exe" "$installPath\tasky.exe"

# PATH 환경변수에 추가
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$newPath = $currentPath + ";" + $installPath
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

Write-Host "설치 완료! 새 PowerShell 창에서 사용하세요."
```

### Linux / macOS

```bash
# 다운로드한 파일에 실행 권한 부여
chmod +x tasky-linux-x64  # Linux의 경우

# 시스템 경로로 이동
sudo mv tasky-linux-x64 /usr/local/bin/tasky

# 설치 확인
tasky --help
```

## 3. 첫 사용법

### 데이터베이스 초기화
```bash
tasky init
```

### 할일 추가
```bash
# 간단한 할일 추가
tasky add "장보기"

# 우선순위와 설명을 포함한 할일
tasky add "회의 준비" --priority high --description "프레젠테이션 자료 준비"

# 마감일이 있는 할일
tasky add "보고서 작성" --due 2024-12-31 --priority medium
```

### 할일 목록 보기
```bash
# 전체 할일 목록
tasky list

# 오늘 마감인 할일만
tasky list --today

# 높은 우선순위 할일만
tasky list --priority high
```

### 할일 완료 처리
```bash
# 할일 완료 (ID는 list 명령어에서 확인)
tasky done 1

# 여러 할일 한번에 완료
tasky done 1 3 5
```

## 4. 주요 명령어

| 명령어 | 설명 | 예시 |
|--------|------|------|
| `add` | 새 할일 추가 | `tasky add "미팅 준비" -p high` |
| `list` | 할일 목록 보기 | `tasky list --today` |
| `done` | 할일 완료 처리 | `tasky done 1` |
| `remove` | 할일 삭제 | `tasky remove 2` |
| `edit` | 할일 수정 | `tasky edit 1 --title "새 제목"` |
| `stats` | 통계 보기 | `tasky stats` |

## 5. 유용한 팁

### 자주 사용하는 명령어 조합
```bash
# 오늘의 긴급한 할일 확인
tasky list --today --priority high

# 완료된 할일 보기
tasky list --status done

# 기한 초과된 할일 확인
tasky list --overdue
```

### 환경설정 (선택사항)
데이터베이스 파일 위치를 변경하려면 환경변수를 설정하세요:

```bash
# Windows (PowerShell)
$env:TASKY_DB_PATH = "C:\MyData\tasky.db"

# Linux/macOS
export TASKY_DB_PATH="$HOME/Documents/tasky.db"
```

## 문제 해결

### "명령을 찾을 수 없습니다" 오류
- 새 터미널/PowerShell 창을 열어보세요
- PATH 환경변수가 올바르게 설정되었는지 확인하세요
- Windows에서는 시스템 재시작이 필요할 수 있습니다

### 데이터베이스 오류
```bash
# 데이터베이스 상태 확인
tasky init --force
```

---

**이제 준비가 완료되었습니다!**
`tasky add "첫 번째 할일"`로 시작해보세요. 더 자세한 사용법은 `tasky --help`를 확인하거나 [사용 가이드](usage.md)를 참고하세요.