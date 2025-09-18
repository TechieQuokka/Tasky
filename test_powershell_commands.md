# PowerShell 명령어 테스트 가이드

사용자가 직접 테스트할 수 있는 PowerShell 명령어들입니다.

## 1. 날짜 형식 테스트

### 오늘 날짜
```powershell
Get-Date -Format 'yyyy-MM-dd'
```

### 내일 날짜
```powershell
(Get-Date).AddDays(1).ToString('yyyy-MM-dd')
```

### 실제 tasky 명령어 테스트
```powershell
# 오늘 할일 추가
tasky add "PowerShell 테스트 1" --due "$(Get-Date -Format 'yyyy-MM-dd')"

# 내일 할일 추가
tasky add "PowerShell 테스트 2" --due "$((Get-Date).AddDays(1).ToString('yyyy-MM-dd'))"

# 목록 확인
tasky list
```

## 2. 월말 계산 테스트

### 이번 달 월말 날짜
```powershell
$month_end = (Get-Date -Day 1).AddMonths(1).AddDays(-1).ToString('yyyy-MM-dd')
Write-Host "이번 달 월말: $month_end"
```

### 실제 사용
```powershell
$month_end = (Get-Date -Day 1).AddMonths(1).AddDays(-1).ToString('yyyy-MM-dd')
tasky add "월간 보고서" --due "$month_end" -p high
```

## 3. 이번 주 월요일/일요일 계산

### 이번 주 월요일
```powershell
$today = Get-Date
$daysSinceMonday = ($today.DayOfWeek.value__ + 6) % 7
$WEEK_START = $today.AddDays(-$daysSinceMonday).ToString('yyyy-MM-dd')
Write-Host "이번 주 월요일: $WEEK_START"
```

### 다음 일요일
```powershell
$today = Get-Date
$daysUntilSunday = (7 - $today.DayOfWeek.value__) % 7
if ($daysUntilSunday -eq 0) { $daysUntilSunday = 7 }
$WEEK_END = $today.AddDays($daysUntilSunday).ToString('yyyy-MM-dd')
Write-Host "다음 일요일: $WEEK_END"
```

## 4. 함수 테스트

### urgent 함수
```powershell
function urgent($title) {
  $tomorrow = (Get-Date).AddDays(1).ToString('yyyy-MM-dd')
  tasky add "$title" -p high --due "$tomorrow"
}

# 사용 예시
urgent "긴급한 작업"
```

## 5. 백업 명령어 테스트

### 백업 파일명 생성
```powershell
$timestamp = Get-Date -Format 'yyyyMMdd_HHmmss'
Write-Host "백업 파일명: tasky_$timestamp.db"
```

### 실제 백업 (경로는 환경에 맞게 조정)
```powershell
$backup_dir = "$env:USERPROFILE\tasky_backups"
New-Item -ItemType Directory -Path $backup_dir -Force

$timestamp = Get-Date -Format 'yyyyMMdd_HHmmss'
# 실제 파일이 있다면 아래 명령어 실행
# Copy-Item "$env:APPDATA\tasky\tasky.db" "$backup_dir\tasky_$timestamp.db"

Write-Host "백업 디렉토리 생성됨: $backup_dir"
```

## 테스트 결과 확인

위 명령어들을 PowerShell에서 실행해보고 결과를 확인해주세요:

1. **날짜 형식**: 올바른 yyyy-MM-dd 형식으로 출력되는지 확인
2. **tasky 명령어**: 실제로 할일이 추가되고 올바른 날짜가 설정되는지 확인
3. **월말/주간 계산**: 정확한 날짜가 계산되는지 확인
4. **함수**: urgent 함수가 정상 작동하는지 확인
5. **백업**: 디렉토리가 생성되고 파일명이 올바른지 확인