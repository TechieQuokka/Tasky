use terminal_size::{Width, terminal_size};
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

fn text_width(text: &str) -> usize {
    text.width()
}

pub fn truncate_text(text: &str, max_width: usize) -> String {
    if text_width(text) <= max_width {
        text.to_string()
    } else {
        // "..." 의 폭은 3
        let target_width = max_width.saturating_sub(3);
        let mut result = String::new();
        let mut current_width = 0;

        for ch in text.chars() {
            let char_width = ch.width().unwrap_or(0);
            if current_width + char_width > target_width {
                break;
            }
            result.push(ch);
            current_width += char_width;
        }

        format!("{}...", result)
    }
}

pub fn truncate_title_for_terminal(title: &str) -> String {
    // 터미널 크기를 감지하고 제목을 적절히 자르기
    if let Some((Width(width), _)) = terminal_size() {
        // 테이블 컬럼별 실제 폭 계산 (유니코드 고려)
        let id_width = text_width("999");                    // ID: 최대 3자리
        let status_width = text_width("⏳ 대기중");          // 상태: 가장 긴 상태
        let priority_width = text_width("🔴 높음");          // 우선순위: 가장 긴 우선순위
        let due_width = text_width("2025-09-19 (10일 전)"); // 마감일: 가장 긴 마감일 표시
        let created_width = text_width("2025-09-19");       // 생성일: 날짜 포맷
        let border_padding = 20;                            // 테이블 보더, 패딩, 여백

        let other_columns_width = id_width + status_width + priority_width + due_width + created_width + border_padding;

        // 제목에 할당할 수 있는 최대 폭 계산
        let available_for_title = if width as usize > other_columns_width {
            let title_space = width as usize - other_columns_width;
            // 제목이 너무 작지 않도록 최소값만 보장 (최대값 제한 제거)
            title_space.max(7)
        } else {
            7 // 터미널이 너무 작은 경우 최소값
        };

        truncate_text(title, available_for_title)
    } else {
        // 터미널 크기를 감지할 수 없는 경우 충분한 크기 사용
        truncate_text(title, 50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("Short", 10), "Short");
        assert_eq!(truncate_text("This is a very long text", 10), "This is...");
        assert_eq!(truncate_text("ABC", 3), "ABC");
        assert_eq!(truncate_text("ABCD", 3), "...");

        // 한글 테스트 (한글은 2칸씩 차지)
        assert_eq!(truncate_text("한글", 4), "한글");
        assert_eq!(truncate_text("한글테스트", 6), "한...");
        assert_eq!(truncate_text("한글테스트", 7), "한글...");
        assert_eq!(truncate_text("한글테스트", 8), "한글...");
    }

    #[test]
    fn test_truncate_title_for_terminal() {
        let long_title = "This is a very long title that should definitely be truncated when displayed in the terminal";
        let result = truncate_title_for_terminal(long_title);
        // 결과가 원본보다 짧아야 함
        assert!(result.len() <= long_title.len());
        if result.len() < long_title.len() {
            assert!(result.ends_with("..."));
        }
    }
}