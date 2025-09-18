# 🚀 Tasky 빠른 시작 가이드

Tasky를 빠르게 설치하고 사용하는 방법을 안내합니다.

## 📦 1. 실행파일 다운로드

[GitHub Releases](https://github.com/TechieQuokka/Tasky/releases/latest)에서 운영체제에 맞는 실행파일을 다운로드하세요:

- **Windows (x64)**: `tasky-windows-x64.exe`
- **Linux (x64)**: `tasky-linux-x64`
- **macOS (Intel)**: `tasky-macos-x64`
- **macOS (Apple Silicon)**: `tasky-macos-arm64`

## 🛠️ 2. 설치

### Windows 설치

```powershell
# 설치 경로 설정 (원하는 경로로 변경)
$installPath = "C:\tools\tasky"
$downloadFile = "$env:USERPROFILE\Downloads\tasky-windows-x64.exe"

# 폴더 생성 및 파일 이동
New-Item -ItemType Directory -Path $installPath -Force
Move-Item $downloadFile "$installPath\tasky.exe"

# PATH 환경변수 추가
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$newPath = $currentPath + ";" + $installPath
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

Write-Host "설치 완료! 새 PowerShell 창에서 'tasky --help' 실행하세요."
```

### Linux/macOS 설치

```bash
# 다운로드 (Linux 예시)
wget https://github.com/TechieQuokka/Tasky/releases/latest/download/tasky-linux-x64

# 실행 권한 부여 및 설치
chmod +x tasky-linux-x64
sudo mv tasky-linux-x64 /usr/local/bin/tasky
```

## 🎯 3. 사용법

```bash
# 데이터베이스 초기화
tasky init

# 할일 추가
tasky add "첫 번째 할일" --priority high --due 2025-01-15

# 할일 목록 보기
tasky list

# 할일 완료 처리
tasky done 1

# 통계 보기
tasky stats
```

## 🔧 4. 환경변수 (선택사항)

데이터베이스 위치를 변경하려면:

```bash
# Windows (PowerShell)
$env:TASKY_DB_PATH = "D:\MyData\tasky.db"

# Linux/macOS
export TASKY_DB_PATH="$HOME/Documents/tasky.db"
```

## 🎉 완료!

이제 `tasky` 명령어로 할일을 관리할 수 있습니다!

자세한 사용법은 [usage.md](usage.md)를 참조하세요.