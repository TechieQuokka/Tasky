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

> ⚠️ **중요**: 아래 경로들은 예시입니다. 사용자가 원하는 경로로 변경하여 사용하세요!

```powershell
# 원하는 설치 경로를 선택하세요 (예시들):
# - "C:\tools\tasky"           (시스템 전체 사용, 권장)
# - "C:\Program Files\tasky"   (프로그램 파일 폴더)
# - "$env:USERPROFILE\tools\tasky"  (사용자 홈 폴더)
# - "D:\Programs\tasky"        (다른 드라이브)

# 1. 선택한 경로에 폴더 생성 (여기서는 C:\tools\tasky 사용)
$installPath = "C:\tools\tasky"  # 👈 이 경로를 원하는 위치로 변경하세요!
New-Item -ItemType Directory -Path $installPath -Force

# 2. 다운로드 파일 경로 확인 (보통 Downloads 폴더)
$downloadPath = "$env:USERPROFILE\Downloads\tasky-windows-x64.exe"  # 👈 실제 다운로드 위치로 변경하세요!

# 3. 파일을 설치 폴더로 이동
Move-Item $downloadPath "$installPath\tasky.exe"

Write-Host "파일이 $installPath 에 저장되었습니다!" -ForegroundColor Green
```

#### 2-2. 환경변수 PATH 설정 (PowerShell)

> 💡 **설명**: PATH에 설치 경로를 추가하면 어느 위치에서나 `tasky` 명령어를 사용할 수 있습니다.

```powershell
# 1. 현재 사용자 PATH 확인
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")

# 2. 새로운 경로 추가 (위에서 설정한 경로와 동일해야 함!)
$installPath = "C:\tools\tasky"  # 👈 2-1에서 사용한 경로와 동일하게 설정하세요!
$newPath = $currentPath + ";" + $installPath

# 3. PATH 환경변수 업데이트
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

# 4. 설정 확인
Write-Host "PATH에 '$installPath'가 추가되었습니다!" -ForegroundColor Green
Write-Host "새 PowerShell 창을 열고 'tasky --help'를 실행해보세요." -ForegroundColor Yellow

# 5. 현재 PATH 확인 (선택사항)
Write-Host "`n현재 PATH:" -ForegroundColor Cyan
$env:PATH -split ';' | Where-Object { $_ -match 'tasky' }
```

> 📝 **일괄 설치 스크립트 예시** (경로만 수정하여 사용):
> ```powershell
> # 설치 경로 설정 (원하는 경로로 변경)
> $installPath = "C:\tools\tasky"
> $downloadFile = "$env:USERPROFILE\Downloads\tasky-windows-x64.exe"
>
> # 폴더 생성 및 파일 이동
> New-Item -ItemType Directory -Path $installPath -Force
> Move-Item $downloadFile "$installPath\tasky.exe"
>
> # PATH 추가
> $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
> $newPath = $currentPath + ";" + $installPath
> [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
>
> Write-Host "설치 완료! 새 PowerShell 창에서 'tasky --help' 실행" -ForegroundColor Green
> ```

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