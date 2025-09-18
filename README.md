# Tasky

## 개요
Tasky는 Rust로 개발된 명령줄 인터페이스(CLI) 기반의 작업 관리 도구입니다.

## 기능
- 작업 생성, 조회, 수정, 삭제
- 우선순위 설정
- 마감일 관리
- 상태 추적 (진행 중, 완료 등)
- SQLite 데이터베이스를 통한 데이터 저장

## 설치
```bash
git clone https://github.com/YOUR_USERNAME/Tasky.git
cd Tasky
cargo build --release
```

## 사용법
```bash
# 작업 추가
cargo run -- add "새로운 작업"

# 작업 목록 조회
cargo run -- list

# 작업 완료 처리
cargo run -- complete <ID>

# 작업 삭제
cargo run -- delete <ID>
```

## 기술 스택
- **언어**: Rust
- **데이터베이스**: SQLite
- **CLI 프레임워크**: clap
- **ORM**: diesel (예상)

## 프로젝트 구조
```
src/
├── cli/           # CLI 명령어 처리
├── models/        # 데이터 모델
├── utils/         # 유틸리티 함수
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