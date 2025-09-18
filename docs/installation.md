# 설치 가이드

다양한 환경에서 Tasky를 설치하는 방법을 설명합니다.

## 📋 시스템 요구사항

### 최소 요구사항
- **운영체제**: Linux, macOS, Windows 10+
- **메모리**: 최소 512MB RAM
- **디스크**: 50MB 여유 공간
- **Rust**: 1.70.0 이상

### 권장 사항
- **운영체제**: Ubuntu 20.04+, macOS 11+, Windows 11
- **메모리**: 1GB RAM 이상
- **터미널**: UTF-8 및 이모지 지원

---

## 🛠️ Rust 설치

Tasky는 Rust로 작성되어 있어 Rust 컴파일러가 필요합니다.

### 1. Rust 설치 (모든 플랫폼)

공식 설치 도구 rustup 사용:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe
```

### 2. 설치 확인

```bash
rustc --version
cargo --version
```

예상 출력:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

---

## 📦 Tasky 설치

### 방법 1: 소스에서 빌드 (권장)

```bash
# 1. 저장소 클론
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky

# 2. 릴리스 빌드
cargo build --release

# 3. 실행 파일 확인
ls target/release/
```

### 방법 2: Cargo를 통한 설치

```bash
# 저장소에서 직접 설치
cargo install --git https://github.com/TechieQuokka/Tasky.git
```

### 방법 3: 개발 빌드

```bash
# 클론 후 개발 모드로 빌드
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky
cargo build
```

---

## 🖥️ 플랫폼별 설치

### Linux (Ubuntu/Debian)

#### 1. 시스템 업데이트
```bash
sudo apt update && sudo apt upgrade -y
```

#### 2. 필수 패키지 설치
```bash
sudo apt install curl build-essential git
```

#### 3. Rust 설치
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 4. Tasky 설치
```bash
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky
cargo build --release
```

#### 5. PATH에 추가 (선택사항)
```bash
# ~/.bashrc 또는 ~/.zshrc에 추가
export PATH="$HOME/Tasky/target/release:$PATH"

# 변경사항 적용
source ~/.bashrc
```

### macOS

#### 1. Xcode Command Line Tools 설치
```bash
xcode-select --install
```

#### 2. Homebrew 설치 (선택사항)
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### 3. Git 설치 확인
```bash
git --version
```

#### 4. Rust 설치
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 5. Tasky 설치
```bash
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky
cargo build --release
```

### Windows

#### 1. Git 설치
- [Git for Windows](https://git-scm.com/download/win) 다운로드 및 설치

#### 2. Visual Studio Build Tools 설치
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) 설치
- 또는 Visual Studio Community와 함께 C++ 빌드 도구 설치

#### 3. Rust 설치
PowerShell을 관리자 권한으로 실행:
```powershell
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe
```

#### 4. 환경 변수 설정
```powershell
# PowerShell 프로필에 추가
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

#### 5. Tasky 설치
```cmd
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky
cargo build --release
```

---

## 🔧 설치 후 설정

### 1. 데이터베이스 초기화

```bash
# 소스에서 빌드한 경우
./target/release/tasky init

# 또는 cargo run 사용
cargo run -- init

# PATH에 추가한 경우
tasky init
```

### 2. 첫 할일 추가

```bash
tasky add "설치 완료 확인"
```

### 3. 설치 확인

```bash
tasky list
tasky stats
```

---

## 🎯 실행 방법

### 1. 직접 실행

```bash
# 릴리스 빌드 실행
./target/release/tasky --help

# 개발 빌드 실행
./target/debug/tasky --help
```

### 2. Cargo 통해 실행

```bash
# 개발 모드
cargo run -- --help

# 릴리스 모드
cargo run --release -- --help
```

### 3. 전역 설치 (선택사항)

#### Linux/macOS
```bash
# 바이너리를 시스템 경로에 복사
sudo cp target/release/tasky /usr/local/bin/

# 또는 심볼릭 링크 생성
sudo ln -s $(pwd)/target/release/tasky /usr/local/bin/tasky

# 사용자 디렉토리에 설치
mkdir -p ~/.local/bin
cp target/release/tasky ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

#### Windows
```cmd
# 시스템 PATH에 디렉토리 추가하거나
# 실행 파일을 PATH의 기존 디렉토리로 복사
copy target\release\tasky.exe C:\Windows\System32\
```

---

## 🔍 설치 문제 해결

### Rust 컴파일 오류

#### 1. Rust 툴체인 업데이트
```bash
rustup update
```

#### 2. 빌드 캐시 정리
```bash
cargo clean
cargo build --release
```

#### 3. 의존성 강제 업데이트
```bash
cargo update
cargo build --release
```

### 링커 오류 (Linux)

```bash
# 필수 빌드 도구 설치
sudo apt install build-essential libc6-dev

# 또는 CentOS/RHEL
sudo yum groupinstall "Development Tools"
```

### 권한 오류

#### Linux/macOS
```bash
# 실행 권한 추가
chmod +x target/release/tasky

# 디렉토리 권한 확인
ls -la ~/.local/share/tasky/
```

#### Windows
- 관리자 권한으로 PowerShell 실행
- 바이러스 백신 소프트웨어 예외 처리 추가

### 데이터베이스 초기화 오류

```bash
# 디렉토리 수동 생성 (Linux/macOS)
mkdir -p ~/.local/share/tasky/

# Windows
mkdir %APPDATA%\tasky

# 강제 초기화
tasky init --force
```

---

## 🔄 업데이트

### 소스에서 설치한 경우

```bash
cd Tasky
git pull origin master
cargo build --release
```

### Cargo로 설치한 경우

```bash
cargo install --git https://github.com/TechieQuokka/Tasky.git --force
```

---

## 🗑️ 제거

### 1. 바이너리 제거

```bash
# 소스 디렉토리 제거
rm -rf Tasky/

# 전역 설치한 경우
sudo rm /usr/local/bin/tasky  # Linux/macOS
del C:\Windows\System32\tasky.exe  # Windows
```

### 2. 데이터 제거 (선택사항)

⚠️ **주의**: 모든 할일 데이터가 삭제됩니다.

```bash
# Linux/macOS
rm -rf ~/.local/share/tasky/

# Windows
rmdir /s %APPDATA%\tasky
```

### 3. Rust 제거 (선택사항)

```bash
rustup self uninstall
```

---

## 📚 추가 자료

- [빠른 시작 가이드](quick-start.md)
- [기본 사용법](usage.md)
- [문제 해결](troubleshooting.md)
- [GitHub 저장소](https://github.com/TechieQuokka/Tasky)