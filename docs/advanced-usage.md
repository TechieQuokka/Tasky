# 고급 사용법

Tasky의 고급 기능과 효율적인 사용법을 다룹니다.

## 📋 설명(Description) 활용

### 작업에 상세 설명 추가하기 ✅ 지원됨

```bash
# 설명과 함께 작업 추가
tasky add "프로젝트 계획" -d "요구사항 분석, 아키텍처 설계, 일정 수립을 포함한 전체적인 프로젝트 계획 수립"

# 여러 줄 설명 추가 (Bash)
tasky add "코드 리뷰" -d "팀원들의 코드를 검토하고 피드백 제공
- 보안 취약점 확인
- 성능 최적화 검토
- 코딩 컨벤션 준수 확인"
```

### 작업 상세 정보 보기 ✅ 지원됨

```bash
# 특정 작업의 모든 정보 보기
tasky show 5

# 출력 예시:
# 📋 할일 상세 정보
# ──────────────────────────────────────────────────
# ID: 5
# 제목: 프로젝트 계획
# 설명: 요구사항 분석, 아키텍처 설계, 일정 수립을 포함한 전체적인 프로젝트 계획 수립
# 상태: ⏳ 대기중
# 우선순위: 🔴 높음
# 마감일: 2025-12-31
# 남은 일수: 45일
# 생성일: 2025-09-18
# 수정일: 2025-09-18
# ──────────────────────────────────────────────────
```

### 상세 목록 보기 ✅ 지원됨

```bash
# 설명을 포함한 상세 목록
tasky list --verbose
tasky list -v

# 조건부 상세 목록
tasky list --status pending --verbose
tasky list --priority high -v
tasky list --today --verbose
```

---

## 🎯 고급 필터링 및 정렬

### 복합 필터 활용

```bash
# 높은 우선순위의 대기 중인 할일
tasky list -s pending -p high

# 마감일 순으로 정렬된 대기 중인 할일
tasky list -s pending --sort due --order asc

# 우선순위 높은 순으로 정렬
tasky list --sort priority --order desc

# 제목 알파벳 순으로 정렬
tasky list --sort title --order asc
```

### 특별 필터 조합

```bash
# 오늘 마감이면서 높은 우선순위
tasky list --today -p high

# 기한 초과된 높은 우선순위 할일
tasky list --overdue -p high

# 긴급한 할일 중 대기 상태만
tasky list --urgent -s pending
```

---

## 📅 날짜 처리 고급 기법

### 다양한 날짜 형식 활용 ✅ 지원됨

```bash
# ISO 형식 ✅
tasky add "ISO 날짜" --due "2024-12-31"

# 미국 형식 ✅
tasky add "미국 날짜" --due "12/31/2024"

# 유럽 형식 ✅
tasky add "유럽 날짜" --due "31/12/2024"

# 자연스러운 형식 ✅
tasky add "자연 날짜" --due "Dec 31, 2024"
tasky add "한국식" --due "31 Dec 2024"

# 전체 월 이름도 지원 ✅
tasky add "전체 월명" --due "December 31, 2024"
tasky add "전체 월명 한국식" --due "31 December 2024"
```

### 날짜 기반 워크플로우 ✅ 지원됨

```bash
# 이번 주 마감일 설정 (월요일 시작) ✅
for day in {1..7}; do
  date_str=$(date -d "+$day days" +%Y-%m-%d)
  tasky add "Day $day 작업" --due "$date_str"
done

# 월말 마감 작업들 ✅
month_end=$(date -d "$(date +%Y-%m-01) +1 month -1 day" +%Y-%m-%d)
tasky add "월간 보고서" --due "$month_end" -p high

# 오늘/내일 작업 간편 추가 ✅
tasky add "오늘 할일" --due $(date +%Y-%m-%d)
tasky add "내일 할일" --due $(date -d "+1 day" +%Y-%m-%d)
```

---

## 🔄 대량 작업 처리

### 스크립트를 통한 일괄 처리 ⚠️ 스크립트 지원

#### 1. 대량 할일 추가
```bash
#!/bin/bash
# bulk_add.sh

# 할일 목록 파일에서 읽어서 추가
while IFS='|' read -r title desc priority due; do
  if [ -n "$due" ]; then
    tasky add "$title" -d "$desc" -p "$priority" --due "$due"
  else
    tasky add "$title" -d "$desc" -p "$priority"
  fi
done < todos.txt
```

할일 목록 파일 (todos.txt):
```
프로젝트 계획|상세한 프로젝트 계획 수립|high|2024-12-25
코드 리뷰|팀원 코드 리뷰 진행|medium|2024-12-23
문서 작성|API 문서 업데이트|low|
```

#### 2. 완료된 할일 일괄 정리
```bash
#!/bin/bash
# cleanup_completed.sh

# 완료된 할일 ID 목록 가져오기
completed_ids=$(tasky list -s done | grep -E "^\s*[0-9]+" | awk '{print $1}' | grep -E "^[0-9]+$")

# 30일 이상 된 완료 할일 삭제 (주의: 실제 날짜 비교 로직 필요)
for id in $completed_ids; do
  echo "완료된 할일 $id 정리 중..."
  # tasky remove $id  # 주의: 실제 실행 전 확인 필요
done
```

### 조건부 업데이트

```bash
#!/bin/bash
# update_urgent.sh

# 기한 초과된 할일들을 높은 우선순위로 변경
overdue_ids=$(tasky list --overdue | grep -E "^\s*[0-9]+" | awk '{print $1}' | grep -E "^[0-9]+$")

for id in $overdue_ids; do
  tasky edit $id -p high
  echo "할일 $id 우선순위를 높음으로 변경"
done
```

---

## 🔧 환경 설정 및 커스터마이징

### 환경 변수 활용

```bash
# ~/.bashrc 또는 ~/.zshrc에 추가

# 기본 우선순위 설정 함수
function tadd() {
  if [ $# -eq 1 ]; then
    tasky add "$1" -p medium
  else
    tasky add "$@"
  fi
}

# 긴급 할일 추가 함수
function urgent() {
  tasky add "$1" -p high --due $(date -d "+1 day" +%Y-%m-%d)
}

# 오늘 할일 빠른 조회
function today() {
  echo "=== 오늘의 할일 ==="
  tasky list --today
  echo ""
  echo "=== 긴급한 할일 ==="
  tasky list --urgent
}
```

### 별칭(Alias) 설정

```bash
# ~/.bashrc 또는 ~/.zshrc에 추가

# 기본 명령어 별칭
alias t='tasky'
alias ta='tasky add'
alias tl='tasky list'
alias td='tasky done'
alias te='tasky edit'
alias tr='tasky remove'
alias ts='tasky stats'

# 자주 사용하는 필터
alias tp='tasky list -s pending'    # 대기 중인 할일
alias tc='tasky list -s done'       # 완료된 할일
alias th='tasky list -p high'       # 높은 우선순위
alias tt='tasky list --today'       # 오늘 할일
alias to='tasky list --overdue'     # 기한 초과
alias tu='tasky list --urgent'      # 긴급
```

---

## 📊 고급 분석 및 리포팅

### 커스텀 리포트 생성

#### 1. 상세 통계 스크립트
```bash
#!/bin/bash
# detailed_stats.sh

echo "=== Tasky 상세 통계 ==="
echo "생성일: $(date)"
echo ""

# 기본 통계
echo "📊 기본 통계"
tasky stats
echo ""

# 우선순위별 분석
echo "🎯 우선순위별 분석"
echo "높은 우선순위: $(tasky list -p high | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo "보통 우선순위: $(tasky list -p medium | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo "낮은 우선순위: $(tasky list -p low | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo ""

# 상태별 분석
echo "📋 상태별 분석"
echo "대기 중: $(tasky list -s pending | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo "완료됨: $(tasky list -s done | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo ""

# 시급성 분석
echo "⚡ 시급성 분석"
echo "오늘 마감: $(tasky list --today | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo "기한 초과: $(tasky list --overdue | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
echo "긴급: $(tasky list --urgent | tail -1 | grep -o '[0-9]\+개' || echo '0개')"
```

#### 2. 주간 리포트 생성
```bash
#!/bin/bash
# weekly_report.sh

WEEK_START=$(date -d "last monday" +%Y-%m-%d)
WEEK_END=$(date -d "next sunday" +%Y-%m-%d)

echo "=== 주간 리포트 ($WEEK_START ~ $WEEK_END) ==="
echo ""

echo "🎯 이번 주 목표"
tasky list --sort due --order asc | head -10
echo ""

echo "⚡ 긴급 처리 필요"
tasky list --urgent
echo ""

echo "📈 주간 통계"
tasky stats
```

---

## 🔄 백업 및 복원

### 자동 백업 스크립트

```bash
#!/bin/bash
# backup_tasky.sh

BACKUP_DIR="$HOME/tasky_backups"
DATE=$(date +%Y%m%d_%H%M%S)
DB_PATH="$HOME/.local/share/tasky/tasky.db"

# 백업 디렉토리 생성
mkdir -p "$BACKUP_DIR"

# 데이터베이스 백업
if [ -f "$DB_PATH" ]; then
  cp "$DB_PATH" "$BACKUP_DIR/tasky_backup_$DATE.db"
  echo "백업 완료: $BACKUP_DIR/tasky_backup_$DATE.db"

  # 30일 이상 된 백업 파일 삭제
  find "$BACKUP_DIR" -name "tasky_backup_*.db" -mtime +30 -delete
  echo "오래된 백업 파일 정리 완료"
else
  echo "데이터베이스 파일을 찾을 수 없습니다: $DB_PATH"
fi
```

### 데이터 내보내기/가져오기

#### 1. JSON 형식으로 내보내기
```bash
#!/bin/bash
# export_json.sh

# 모든 할일을 JSON 형식으로 내보내기 (수동 구현 필요)
echo "{"
echo "  \"export_date\": \"$(date -Iseconds)\","
echo "  \"todos\": ["

# 할일 목록을 JSON 형태로 변환하는 로직
# (실제로는 Tasky에 export 기능 추가 필요)

echo "  ]"
echo "}"
```

#### 2. CSV 형식으로 내보내기
```bash
#!/bin/bash
# export_csv.sh

echo "ID,Status,Priority,Title,Description,Due_Date,Created_At" > tasky_export.csv

# tasky list 출력을 파싱하여 CSV로 변환
# (실제 구현 시 더 정교한 파싱 필요)
tasky list | grep -E "^\s*[0-9]+" | while read line; do
  # CSV 형식으로 변환하는 로직
  echo "$line" >> tasky_export.csv
done

echo "CSV 파일로 내보내기 완료: tasky_export.csv"
```

---

## 🤖 자동화 및 통합

### Cron을 통한 자동화

```bash
# crontab -e에 추가

# 매일 오전 9시에 오늘 할일 알림
0 9 * * * /usr/local/bin/tasky list --today | mail -s "오늘의 할일" user@example.com

# 매주 월요일 오전 8시에 주간 리포트
0 8 * * 1 /home/user/scripts/weekly_report.sh | mail -s "주간 할일 리포트" user@example.com

# 매일 자정에 백업
0 0 * * * /home/user/scripts/backup_tasky.sh

# 매주 일요일에 완료된 할일 정리
0 23 * * 0 /home/user/scripts/cleanup_completed.sh
```

### Git Hook 통합

```bash
#!/bin/bash
# .git/hooks/pre-commit

# 커밋 전에 관련 할일이 있는지 확인
commit_msg=$(cat .git/COMMIT_EDITMSG 2>/dev/null || echo "")

if echo "$commit_msg" | grep -q "#[0-9]\+"; then
  task_id=$(echo "$commit_msg" | grep -o "#[0-9]\+" | head -1 | sed 's/#//')

  if tasky list | grep -q "^\s*$task_id\s"; then
    echo "할일 #$task_id 관련 커밋입니다."
    # 원한다면 자동으로 할일 완료 처리
    # tasky done $task_id
  fi
fi
```

---

## 🎨 출력 커스터마이징

### 색상 및 형식 변경

환경 변수를 통한 색상 제어:
```bash
# ~/.bashrc에 추가

# 컬러 출력 비활성화
export NO_COLOR=1

# 또는 특정 색상 스킴 사용
export TASKY_COLOR_SCHEME=dark
```

### 커스텀 출력 포맷터

```bash
#!/bin/bash
# custom_format.sh

# 간단한 형식으로 출력
function tl_simple() {
  tasky list | grep -E "^\s*[0-9]+" | while read line; do
    id=$(echo "$line" | awk '{print $1}')
    title=$(echo "$line" | cut -d'│' -f4 | sed 's/^\s*\|\s*$//g')
    echo "[$id] $title"
  done
}

# 우선순위만 표시
function tl_priority() {
  echo "=== 높은 우선순위 ==="
  tasky list -p high
  echo ""
  echo "=== 보통 우선순위 ==="
  tasky list -p medium
  echo ""
  echo "=== 낮은 우선순위 ==="
  tasky list -p low
}
```

---

## 🔍 고급 검색 및 필터링

### 복잡한 검색 패턴

```bash
# 제목에 특정 키워드가 포함된 할일 찾기
tasky list | grep -i "프로젝트"

# 여러 조건을 만족하는 할일
tasky list -p high | grep -E "(긴급|중요)"

# 정규표현식을 사용한 검색
tasky list | grep -E "\[.*\]"  # 대괄호가 포함된 제목
```

### 날짜 범위 검색 (스크립트)

```bash
#!/bin/bash
# date_range_search.sh

start_date="$1"
end_date="$2"

if [ -z "$start_date" ] || [ -z "$end_date" ]; then
  echo "사용법: $0 <시작날짜> <끝날짜>"
  echo "예시: $0 2024-12-01 2024-12-31"
  exit 1
fi

echo "기간: $start_date ~ $end_date"
echo ""

# 해당 기간의 할일 검색 (수동 필터링)
tasky list --sort due --order asc | while read line; do
  if echo "$line" | grep -E "[0-9]{4}-[0-9]{2}-[0-9]{2}" >/dev/null; then
    due_date=$(echo "$line" | grep -o "[0-9]{4}-[0-9]{2}-[0-9]{2}")
    if [[ "$due_date" >= "$start_date" && "$due_date" <= "$end_date" ]]; then
      echo "$line"
    fi
  fi
done
```

---

## 💡 생산성 향상 팁

### 시간 관리 테크닉

#### 1. 포모도로 기법 통합
```bash
#!/bin/bash
# pomodoro.sh

TASK_ID="$1"
DURATION=${2:-25}  # 기본 25분

if [ -z "$TASK_ID" ]; then
  echo "사용법: $0 <할일_ID> [분단위_시간]"
  exit 1
fi

echo "포모도로 시작: 할일 #$TASK_ID ($DURATION분)"
sleep $((DURATION * 60))
echo "포모도로 완료! 할일 #$TASK_ID 진행 상황을 확인하세요."

# 옵션: 자동으로 할일 완료 처리
read -p "이 할일을 완료 처리하시겠습니까? (y/N): " answer
if [[ "$answer" =~ ^[Yy]$ ]]; then
  tasky done "$TASK_ID"
fi
```

#### 2. GTD (Getting Things Done) 워크플로우
```bash
# GTD 컨텍스트별 할일 관리

# 상황별 할일 추가
function gtd_add() {
  context="$1"
  shift
  tasky add "[$context] $*" -d "GTD 컨텍스트: $context"
}

# 사용 예시
gtd_add "전화" "클라이언트에게 연락"
gtd_add "컴퓨터" "이메일 정리"
gtd_add "외출" "은행 업무"
```

### 팀 협업

#### 1. 할일 태그 시스템
```bash
# 태그를 제목에 포함하여 관리
tasky add "[팀미팅] 스프린트 계획 논의"
tasky add "[개발] API 엔드포인트 구현"
tasky add "[리뷰] 코드 리뷰 진행"

# 태그별 검색
function search_tag() {
  tag="$1"
  tasky list | grep "\[$tag\]"
}
```

#### 2. 프로젝트별 관리
```bash
# 프로젝트 이름을 접두사로 사용
tasky add "프로젝트A: 요구사항 분석"
tasky add "프로젝트A: 프로토타입 개발"
tasky add "프로젝트B: 디자인 검토"

# 프로젝트별 검색
function project_todos() {
  project="$1"
  tasky list | grep "$project:"
}
```

---

이러한 고급 기능들을 활용하여 Tasky를 더욱 효율적으로 사용할 수 있습니다. 필요에 따라 스크립트를 수정하고 자신만의 워크플로우를 구축해보세요.