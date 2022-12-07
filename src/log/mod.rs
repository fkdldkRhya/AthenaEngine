use std::fmt::Error;
use chrono::{Datelike, DateTime, Local, Timelike};

/// 로그 출력 태그
pub enum LogTypeTag {
    INFO,
    DEBUG,
    WARNING,
    FATAL
}


/// 엔진 로그 출력 관리자
///
/// # Examples
///
/// ```
/// log::log_writer(String::from("This is error!"), String::from("MAIN"), LogTypeTag.INFO)
/// log::log_writer(String::from("This is error!"), String::from("main"), LogTypeTag.DEBUG)
/// log::log_writer(String::from("This is error!"), String::from("main"), LogTypeTag.WARNING)
/// ```
///
/// # Argument
/// message : 로그 내용
///
/// log_tag : 로그 태그
///
/// # Return
/// 입력한 내용을 기반으로 작성한 로그
pub fn log_text_writer(message : String, writer : String, log_tag : LogTypeTag) -> String {
    // 로그 작성 날짜
    let local: DateTime<Local> = Local::now();
    let now_date = format!("{}/{}/{} {:0>2}-{:0>2}-{:0>2}",local.year(), local.month(), local.day(), local.hour(), local.minute(), local.second());

    // 로그 태그 설정
    let log_tag = match log_tag {
        LogTypeTag::INFO => "INFO",
        LogTypeTag::DEBUG => "DEBUG",
        LogTypeTag::WARNING => "WARNING",
        LogTypeTag::FATAL => "FATAL"
    };

    // 로그 작성
    return format!("{:<19} {:<5} [{}] {:>10}", now_date, log_tag, writer.to_uppercase(), message);
}