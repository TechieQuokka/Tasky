# 🚀 Tasky 빠른 시작 가이드

Tasky를 빠르게 설치하고 사용하는 방법을 안내합니다.

## 📦 1. 실행파일 다운로드

[GitHub Releases](https://github.com/TechieQuokka/Tasky/releases/latest)에서 운영체제에 맞는 실행파일을 다운로드하세요:

- **Windows (x64)**: `tasky-windows-x64.exe`
- **Linux (x64)**: `tasky-linux-x64`
- **macOS (Intel)**: `tasky-macos-x64`
- **macOS (Apple Silicon)**: `tasky-macos-arm64`

## 🛠️ 2. 설치 및 PATH 설정

### Windows 설치

#### 2-1. 실행파일 저장
```powershell
# 추천 위치에 폴더 생성
New-Item -ItemType Directory -Path "C:\tools\tasky" -Force

# 다운로드한 파일을 해당 폴더로 이동
Move-Item "다운로드경로\tasky-windows-x64.exe" "C:\tools\tasky\tasky.exe"
```

#### 2-2. 환경변수 PATH 설정 (PowerShell)
```powershell
# 현재 사용자 PATH 확인
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")

# 새로운 경로 추가 (C:\tools\tasky 예시)
$newPath = $currentPath + ";C:\tools\tasky"

# PATH 환경변수 업데이트
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

# 설정 확인
Write-Host "PATH가 업데이트되었습니다!"
Write-Host "새 PowerShell 창을 열고 'tasky --help'를 실행해보세요."
```

#### 2-3. GUI를 통한 PATH 설정 (대안)
1. `Win + R` → `sysdm.cpl` 실행
2. **고급** 탭 → **환경 변수** 클릭
3. **사용자 변수**에서 `PATH` 선택 → **편집**
4. **새로 만들기** → `C:\tools\tasky` 추가
5. **확인** → **확인**

### Linux/macOS 설치

```bash
# 다운로드 (Linux 예시)
wget https://github.com/TechieQuokka/Tasky/releases/latest/download/tasky-linux-x64

# 실행 권한 부여
chmod +x tasky-linux-x64

# 시스템 전체 설치 (권장)
sudo mv tasky-linux-x64 /usr/local/bin/tasky

# 또는 개인 사용자 설치
mkdir -p ~/.local/bin
mv tasky-linux-x64 ~/.local/bin/tasky

# .bashrc 또는 .zshrc에 PATH 추가 (개인 설치 시)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 🎯 3. 설치 확인

새로운 명령 프롬프트/터미널을 열고 다음 명령어를 실행하세요:

```bash
# 설치 확인
tasky --help

# 데이터베이스 초기화
tasky init

# 첫 번째 할일 추가
tasky add "첫 번째 할일" --priority high

# 할일 목록 확인
tasky list
```

## 📚 4. 기본 사용법

### 할일 관리
```bash
# 할일 추가
tasky add "회의 준비" --priority medium --due 2025-01-15 --description "프로젝트 진행 상황 정리"

# 할일 목록 보기
tasky list                    # 전체 목록
tasky list --status pending  # 대기 중인 할일만
tasky list --today           # 오늘 마감인 할일
tasky list --urgent          # 긴급한 할일

# 할일 완료 처리
tasky done 1

# 할일 수정
tasky edit 2 --title "수정된 제목" --priority low

# 할일 삭제
tasky remove 3
```

### 통계 및 정보
```bash
# 전체 통계 보기
tasky stats

# 데이터베이스 정보 확인
tasky db-info
```

## 🔧 5. 환경설정

### 데이터베이스 위치 변경
```bash
# Windows (PowerShell)
$env:TASKY_DB_PATH = "D:\MyData\tasky.db"

# Linux/macOS
export TASKY_DB_PATH="$HOME/Documents/tasky.db"

# 영구 설정 (Linux/macOS)
echo 'export TASKY_DB_PATH="$HOME/Documents/tasky.db"' >> ~/.bashrc
```

### 데이터베이스 초기화
```bash
# 새로 초기화
tasky init

# 강제 초기화 (기존 데이터 삭제)
tasky init --force
```

## 🆘 6. 문제 해결

### 명령어를 찾을 수 없음
- PowerShell/명령 프롬프트를 재시작
- PATH 설정이 올바른지 확인: `echo $env:PATH` (PowerShell) 또는 `echo $PATH` (Linux/macOS)

### 데이터베이스 오류
```bash
# 데이터베이스 상태 확인
tasky db-info

# 문제가 있으면 재초기화
tasky init --force
```

### 권한 오류 (Linux/macOS)
```bash
# 실행 권한 추가
chmod +x ~/.local/bin/tasky

# 또는 sudo로 시스템 전체 설치
sudo mv tasky-linux-x64 /usr/local/bin/tasky
```

## 🎉 설치 완료!

이제 어디서든 `tasky` 명령어로 할일을 관리할 수 있습니다!

자세한 사용법은 [README.md](README.md)를 참조하세요.