use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};

use crate::error::{Result, TaskyError};

pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    // 다양한 날짜 형식을 지원
    let date_formats = vec![
        "%Y-%m-%d",        // ISO 형식: 2024-12-31
        "%Y/%m/%d",        // ISO 슬래시 형식: 2024/12/31
        "%Y.%m.%d",        // ISO 점 형식: 2024.12.31
        "%m/%d/%Y",        // 미국 형식: 12/31/2024
        "%d/%m/%Y",        // 유럽 형식: 31/12/2024
        "%b %d, %Y",       // 자연스러운 형식: Dec 31, 2024
        "%d %b %Y",        // 한국식: 31 Dec 2024
        "%B %d, %Y",       // 전체 월 이름: December 31, 2024
        "%d %B %Y",        // 전체 월 이름 한국식: 31 December 2024
    ];

    let mut naive_date = None;
    for format in &date_formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            naive_date = Some(date);
            break;
        }
    }

    let naive_date = naive_date
        .ok_or_else(|| TaskyError::InvalidDateFormat { date: date_str.to_string() })?;

    let local_datetime = Local
        .from_local_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .ok_or_else(|| TaskyError::InvalidDateFormat { date: date_str.to_string() })?;

    Ok(local_datetime.with_timezone(&Utc))
}

pub fn format_datetime(dt: &DateTime<Utc>) -> String {
  let local = dt.with_timezone(&Local);
  local.format("%Y년 %m월 %d일 %H:%M").to_string()
}

pub fn format_date(dt: &DateTime<Utc>) -> String {
  let local = dt.with_timezone(&Local);
  local.format("%Y-%m-%d").to_string()
}

pub fn format_relative_time(dt: &DateTime<Utc>) -> String {
  let now = Utc::now();
  let duration = dt.signed_duration_since(now);

  if duration.num_days() > 0 {
    format!("{}일 후", duration.num_days())
  } else if duration.num_days() < 0 {
    format!("{}일 전", -duration.num_days())
  } else if duration.num_hours() > 0 {
    format!("{}시간 후", duration.num_hours())
  } else if duration.num_hours() < 0 {
    format!("{}시간 전", -duration.num_hours())
  } else if duration.num_minutes() > 0 {
    format!("{}분 후", duration.num_minutes())
  } else if duration.num_minutes() < 0 {
    format!("{}분 전", -duration.num_minutes())
  } else {
    "지금".to_string()
  }
}

pub fn today_start() -> DateTime<Utc> {
  let local_today = Local::now().date_naive();
  let local_start = Local
      .from_local_datetime(&local_today.and_hms_opt(0, 0, 0).unwrap())
      .single()
      .unwrap();
  local_start.with_timezone(&Utc)
}

pub fn today_end() -> DateTime<Utc> {
  let local_today = Local::now().date_naive();
  let local_end = Local
      .from_local_datetime(&local_today.and_hms_opt(23, 59, 59).unwrap())
      .single()
      .unwrap();
  
  local_end.with_timezone(&Utc)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_date() {
    // ISO 형식
    let result = parse_date("2024-01-15");
    assert!(result.is_ok());

    // ISO 슬래시 형식
    let result = parse_date("2024/12/31");
    assert!(result.is_ok());

    // ISO 점 형식
    let result = parse_date("2024.10.10");
    assert!(result.is_ok());

    // 미국 형식
    let result = parse_date("12/31/2024");
    assert!(result.is_ok());

    // 유럽 형식
    let result = parse_date("31/12/2024");
    assert!(result.is_ok());

    // 자연스러운 형식
    let result = parse_date("Dec 31, 2024");
    assert!(result.is_ok());

    // 한국식
    let result = parse_date("31 Dec 2024");
    assert!(result.is_ok());

    // 전체 월 이름
    let result = parse_date("December 31, 2024");
    assert!(result.is_ok());

    // 잘못된 형식
    let result = parse_date("invalid");
    assert!(result.is_err());
  }

  #[test]
  fn test_format_relative_time() {
    let now = Utc::now();
    let tomorrow = now + chrono::Duration::days(1);
    let yesterday = now - chrono::Duration::days(1);

    assert!(format_relative_time(&tomorrow).contains("후"));
    assert!(format_relative_time(&yesterday).contains("전"));
    assert_eq!(format_relative_time(&now), "지금");
  }
}