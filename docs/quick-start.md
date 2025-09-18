# 빠른 시작 가이드

Tasky를 5분 안에 시작해보세요!

## 1. 설치

### 소스에서 빌드 (권장)
```bash
git clone https://github.com/TechieQuokka/Tasky.git
cd Tasky
cargo build --release
```

### 실행 파일 생성
```bash
# 실행 파일은 target/release/tasky 에 생성됩니다
./target/release/tasky --help
```

## 2. 초기 설정

처음 사용하기 전에 데이터베이스를 초기화하세요:

```bash
# 데이터베이스 초기화
cargo run -- init

# 또는 실행 파일 사용 시
./target/release/tasky init
```

## 3. 첫 할일 추가

```bash
# 간단한 할일 추가
cargo run -- add "첫 번째 할일"

# 설명과 우선순위를 포함한 할일
cargo run -- add "중요한 미팅 준비" -d "프레젠테이션 자료 준비" -p high

# 마감일이 있는 할일
cargo run -- add "프로젝트 제출" -d "최종 보고서 작성" -p high --due "2024-12-31"
```

## 4. 할일 목록 조회

```bash
# 전체 할일 목록
cargo run -- list

# 오늘 할일만 보기
cargo run -- list --today

# 기한 초과된 할일 보기
cargo run -- list --overdue

# 긴급한 할일 보기 (높은 우선순위 + 마감일 임박)
cargo run -- list --urgent
```

## 5. 할일 완료 처리

```bash
# 할일 완료 (ID는 list 명령어에서 확인)
cargo run -- done 1

# 완료된 할일을 다시 대기 상태로
cargo run -- undone 1
```

## 6. 통계 확인

```bash
# 전체 통계 보기
cargo run -- stats
```

## 다음 단계

이제 기본 사용법을 익혔습니다! 더 자세한 내용은:

- [기본 사용법](usage.md) - 모든 명령어와 옵션
- [고급 사용법](advanced-usage.md) - 고급 기능과 팁
- [예제 모음](examples.md) - 실제 사용 시나리오

## 자주 사용하는 명령어

```bash
# 오늘의 할일 확인
cargo run -- list --today

# 긴급한 할일 확인
cargo run -- list --urgent

# 높은 우선순위 할일만 보기
cargo run -- list -p high

# 완료된 할일 보기
cargo run -- list -s done

# 전체 통계 확인
cargo run -- stats
```