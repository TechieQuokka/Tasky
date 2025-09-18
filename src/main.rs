use clap::Parser;
use colored::*;

use tasky::cli::{Cli, execute};

fn main() {
    // 컬러 출력 초기화 (Windows 지원)
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap_or(());
    
    // CLI 파싱
    let cli = Cli::parse();
    
    // 명령어 실행
    match execute(cli.command) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {}", "오류:".red().bold(), e);
            std::process::exit(1);
        }
    }
}