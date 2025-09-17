use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};

use crate::error::{Result, TaskyError};

pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {

  let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
      .map_err(|_| TaskyError::InvalidDateFormat { date: date_str.to_string(), })?;

  
  let local_datetime = Local
      .from_local_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap())
      .single()
      .ok_or_else(|| TaskyError::InvalidDateFormat { date: date_str.to_string(), })?;
  
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
    let result = parse_date("2024-01-15");
    assert!(result.is_ok());

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