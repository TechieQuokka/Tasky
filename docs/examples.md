# 실제 사용 예제

다양한 상황에서 Tasky를 활용하는 실제 예제들을 모았습니다.

## 📅 일일 할일 관리

### 아침 루틴
```bash
# 1. 오늘 할일 확인
tasky list --today

# 2. 긴급한 일 확인
tasky list --urgent

# 3. 새로운 할일 추가
tasky add "이메일 확인" -p low
tasky add "팀 미팅 참석" -p medium --due "2024-12-20 14:00"
tasky add "프로젝트 보고서 작성" -p high --due "2024-12-20"
```

### 저녁 정리
```bash
# 1. 완료한 일들 처리
tasky done 1  # 이메일 확인 완료
tasky done 2  # 팀 미팅 완료

# 2. 전체 진행 상황 확인
tasky stats

# 3. 내일 할일 미리 추가
tasky add "클라이언트 미팅 준비" -p high --due "2024-12-21"
```

---

## 🏢 프로젝트 관리

### 새 프로젝트 시작
```bash
# 프로젝트 관련 할일들 추가
tasky add "프로젝트 요구사항 분석" -d "클라이언트와 상세 요구사항 논의" -p high --due "2024-12-22"
tasky add "기술 스택 선정" -d "팀과 함께 적합한 기술 스택 논의 및 결정" -p medium --due "2024-12-23"
tasky add "개발 환경 설정" -d "로컬 및 배포 환경 구성" -p medium --due "2024-12-24"
tasky add "UI/UX 디자인 검토" -d "디자이너와 함께 최종 디자인 검토" -p low --due "2024-12-25"
```

### 프로젝트 진행 상황 추적
```bash
# 높은 우선순위 작업 확인
tasky list -p high

# 마감일 순으로 정렬하여 확인
tasky list --sort due --order asc

# 완료된 작업 확인
tasky list -s done
```

---

## 📚 학습 및 개발

### 새로운 기술 학습
```bash
# 학습 계획 수립
tasky add "Rust 기초 문법 학습" -d "변수, 함수, 구조체 등 기본 개념 학습" -p high --due "2024-12-25"
tasky add "Rust 프로젝트 실습" -d "간단한 CLI 도구 만들어보기" -p medium --due "2024-12-30"
tasky add "Rust 고급 개념 학습" -d "소유권, 라이프타임, 트레이트 등" -p low --due "2025-01-05"

# 일일 학습 목표
tasky add "오늘의 Rust 학습" -d "1시간 동안 공식 문서 읽기" -p medium --due $(date +%Y-%m-%d)
```

### 코드 리뷰 및 개선
```bash
# 코드 품질 관리
tasky add "코드 리뷰 진행" -d "팀원들의 PR 검토 및 피드백" -p high
tasky add "리팩토링 계획 수립" -d "기술 부채 정리 및 개선 방안 논의" -p medium
tasky add "테스트 코드 작성" -d "단위 테스트 및 통합 테스트 추가" -p medium
```

---

## 💼 업무 관리

### 회의 관리
```bash
# 회의 전
tasky add "회의 안건 준비" -d "주간 팀 미팅 안건 정리" -p high --due "2024-12-20 09:00"
tasky add "회의 자료 준비" -d "프레젠테이션 슬라이드 작성" -p high --due "2024-12-20 10:00"

# 회의 후
tasky add "회의록 작성" -d "주요 결정사항 및 액션 아이템 정리" -p medium
tasky add "액션 아이템 할일 등록" -d "회의에서 나온 액션 아이템들을 개별 할일로 등록" -p medium
```

### 이메일 및 커뮤니케이션
```bash
# 정기적인 커뮤니케이션
tasky add "이메일 답장" -d "중요한 이메일들 우선 답장" -p high
tasky add "슬랙 메시지 확인" -d "놓친 중요한 메시지 확인 및 답장" -p medium
tasky add "클라이언트 업데이트" -d "프로젝트 진행 상황 클라이언트에게 보고" -p high --due "2024-12-20"
```

---

## 🔄 정기적인 작업

### 주간 루틴
```bash
# 월요일 - 주간 계획
tasky add "주간 목표 설정" -d "이번 주 달성하고자 하는 목표들 정리" -p high
tasky add "우선순위 할일 정리" -d "이번 주 중요한 일들 우선순위 매기기" -p medium

# 금요일 - 주간 정리
tasky add "주간 회고" -d "이번 주 성과 및 개선점 정리" -p medium
tasky add "다음 주 준비" -d "다음 주 중요한 일정 미리 확인" -p low
```

### 월간 루틴
```bash
# 월초
tasky add "월간 목표 설정" -d "이번 달 개인 및 업무 목표 설정" -p high --due "2024-12-03"
tasky add "지난 달 성과 정리" -d "지난 달 성과 및 학습한 내용 정리" -p medium

# 월말
tasky add "월간 회고" -d "이번 달 목표 달성도 및 개선점 분석" -p medium
tasky add "다음 달 계획" -d "다음 달 주요 일정 및 목표 계획" -p medium
```

---

## 🎯 고급 활용법

### 복잡한 프로젝트 관리
```bash
# 대규모 프로젝트를 단계별로 분해
tasky add "[Phase 1] 요구사항 분석" -d "사용자 스토리 작성 및 기능 명세서 작성" -p high --due "2024-12-25"
tasky add "[Phase 1] 시스템 설계" -d "아키텍처 설계 및 DB 스키마 설계" -p high --due "2024-12-27"
tasky add "[Phase 2] 백엔드 개발" -d "API 서버 개발 및 데이터베이스 구축" -p high --due "2025-01-10"
tasky add "[Phase 2] 프론트엔드 개발" -d "사용자 인터페이스 및 사용자 경험 구현" -p high --due "2025-01-15"
tasky add "[Phase 3] 테스트 및 배포" -d "통합 테스트 및 프로덕션 배포" -p high --due "2025-01-20"
```

### 우선순위별 워크플로우
```bash
# 긴급하고 중요한 일 (높은 우선순위)
tasky list -p high
tasky add "시스템 장애 대응" -p high

# 중요하지만 긴급하지 않은 일 (보통 우선순위)
tasky add "코드 리팩토링" -p medium --due "2024-12-30"
tasky add "문서 업데이트" -p medium

# 긴급하지 않고 중요하지도 않은 일 (낮은 우선순위)
tasky add "사무용품 정리" -p low
tasky add "개인 블로그 포스팅" -p low
```

---

## 📊 분석 및 개선

### 생산성 분석
```bash
# 정기적인 통계 확인
tasky stats

# 완료된 작업 분석
tasky list -s done --sort updated --order desc

# 기한 초과된 작업 분석
tasky list --overdue
```

### 시간 관리 개선
```bash
# 오늘 집중할 작업들
tasky list --today --sort priority --order desc

# 이번 주 마감인 작업들
tasky list --sort due --order asc

# 시간이 많이 걸리는 작업들은 세분화
tasky edit 5 -t "[1/3] 보고서 개요 작성"
tasky add "[2/3] 보고서 본문 작성" -p high --due "2024-12-21"
tasky add "[3/3] 보고서 검토 및 최종 제출" -p high --due "2024-12-22"
```

---

## 🛠️ 팀 워크플로우

### 팀 프로젝트 관리 (개인 부분)
```bash
# 자신이 담당하는 부분만 관리
tasky add "[Backend] 사용자 인증 API" -d "JWT 기반 로그인/로그아웃 API 개발" -p high --due "2024-12-23"
tasky add "[Backend] 데이터베이스 최적화" -d "쿼리 성능 개선 및 인덱스 추가" -p medium --due "2024-12-25"
tasky add "[Review] 프론트엔드 코드 리뷰" -d "팀원의 React 컴포넌트 코드 리뷰" -p medium

# 팀 미팅 관련
tasky add "스프린트 계획 미팅" -p high --due "2024-12-20 10:00"
tasky add "일일 스탠드업" -p medium --due $(date +%Y-%m-%d)" 09:30"
```

### 지식 공유
```bash
# 팀 내 지식 공유
tasky add "기술 블로그 포스팅" -d "Rust 성능 최적화 경험 공유" -p low --due "2024-12-30"
tasky add "팀 내 발표 준비" -d "새로운 라이브러리 도입 제안 발표" -p medium --due "2024-12-25"
tasky add "문서화 개선" -d "API 문서 및 개발 가이드 업데이트" -p medium
```

---

## 💡 추가 팁

### 효과적인 제목 작성
```bash
# ❌ 나쁜 예
tasky add "작업"
tasky add "미팅"

# ✅ 좋은 예
tasky add "클라이언트 요구사항 분석 미팅"
tasky add "사용자 인증 API 테스트 코드 작성"
```

### 설명 활용하기
```bash
# 상세한 설명으로 나중에 참고하기 쉽게
tasky add "성능 최적화" -d "DB 쿼리 실행 시간을 50% 단축하기 위한 인덱스 추가 및 쿼리 개선. 특히 사용자 검색 API의 응답 시간 개선에 집중."

# 체크리스트 형태로 활용
tasky add "배포 준비" -d "1. 코드 리뷰 완료 2. 테스트 통과 확인 3. 배포 스크립트 검증 4. 모니터링 설정 확인"
```

### 정기적인 정리
```bash
# 주기적으로 완료된 할일 확인하여 성취감 느끼기
tasky list -s done --sort updated --order desc

# 기한 초과된 할일들 재검토
tasky list --overdue

# 너무 오래된 할일들 정리
tasky list --sort created --order asc
```