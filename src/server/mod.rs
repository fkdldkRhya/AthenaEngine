use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::time::Duration;

use crate::server::request_parser::request_parser::{Request, request_parser};
use crate::server::response_parser::response_writer::{Response, response_parser};
use crate::log::{log_more_text_writer, log_text_writer, LogTypeTag};

/// Public module - request_parser
pub mod request_parser;
/// Public module - response_parser
pub mod response_parser;
/// Public module - page_manager
pub mod page_manager;
/// Public module - session_manager
mod session_manager;


/// 현재 파일 정보 반환
fn get_this_name() -> String{
    return String::from("main/server");
}


/// 클라이언트 작업 성공 여부
enum TaskSuccess {
    Success,
    Error
}

/// 클라이언트 접속 이벤트 Handler
pub type RequestHandler = Box<dyn Send + (Fn(&Request)) + 'static>;
pub type ResponseHandler = Box<dyn Send + (Fn(&Request) -> Response) + 'static>;
/// 클라이언트 접속 이벤트 Struct
pub struct ClientEvent {
    pub event_request : Option<RequestHandler>,
    pub event_response : Option<ResponseHandler>
}

/// 클라이언트 접속 이벤트
pub static mut EVENT: ClientEvent = ClientEvent {
    event_request: None,
    event_response: None
};


/// Thread 실행 인자
pub struct ThreadTaskArgs {
    pub tcp_stream : TcpStream,
}


/// Athena Engine 서버 시작 함수
///
/// # Examples
///
/// ```
/// server::start_server("127.0.0.1", 8080)
/// ```
///
/// # Argument
/// server_ip : 서버 IP 주소
///
/// server_port : 서버 Port 번호 (0 ~ 65535)
pub fn start_server(server_ip : String, server_port : u16) {
    // TCP 리스너 생성
    let listener = TcpListener::bind(format!("{}:{}", server_ip, server_port)).unwrap();
    // TCP 연결 대기
    for stream in listener.incoming() {
        // Stream 추출
        match stream {
            Ok(stream) => { // 작업 성공!
                // Timeout 설정 - read_timeout
                match &stream.set_read_timeout(Some(Duration::from_secs(15))) {
                    Ok(_) => {}
                    Err(error) => {
                        // 오류 로그 작성
                        println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                    }
                };
                // Timeout 설정 - read_timeout
                match &stream.set_write_timeout(Some(Duration::from_secs(15))) {
                    Ok(_) => {}
                    Err(error) => {
                        // 오류 로그 작성
                        println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                    }
                };
                // 인자 데이터 설정
                let arg : ThreadTaskArgs = ThreadTaskArgs {
                    tcp_stream: stream
                };
                // Thread 로 작업 전송
                std::thread::spawn(move || {
                    handle_connection(arg);
                });
            },
            Err(error) =>  { // 작업 실패, 예외 처리
                // 오류 로그 작성
                println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
            }
        };
    }
}


/// Athena Engine Client 접근 처리 함수
fn handle_connection(mut threadPoolArgs: ThreadTaskArgs) {
    // 작업 성공 여부
    let mut task_success = TaskSuccess::Success;
    // HTTP 요청 읽기
    let mut buffer = [0; 1024];
    match &threadPoolArgs.tcp_stream.read(&mut buffer) {
        Ok(_) => {},
        Err(error) => {
            task_success = TaskSuccess::Error;

            // 로그 출력
            println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
        }
    }
    // Request 데이터
    let binding = String::from_utf8_lossy(&buffer);
    let mut http_request : Vec<&str> = binding.split("\r\n").collect();
    // 클라이언트 IP 주소
    let client_ip : String = match threadPoolArgs.tcp_stream.peer_addr() {
        Ok(value) => value.ip().to_string(),
        Err(_) => String::from("(NoIPAddress)")
    };
    // 작업 성공 여부 비교
    match task_success {
        _Success => { // 작업 성공
            // Request 패킷 분석
            let request = request_parser(&http_request);
            // Request 이벤트 실행
            unsafe {
                match &EVENT.event_request {
                    Some(Box) => {
                        // 로그 출력
                        println!("{}", log_more_text_writer(String::from("Run request EVENT handler."), get_this_name(), LogTypeTag::INFO, format!("IP:{}", client_ip)));

                        // 이벤트 실행
                        Box(&request);
                    },
                    None => {
                        // 로그 출력
                        println!("{}", log_more_text_writer(String::from("Request EVENT handler failed, no registered EVENT."), get_this_name(), LogTypeTag::INFO, format!("IP: {}", client_ip)));
                    }
                }
            }
            // Response 이벤트 실행
            unsafe {
                match &EVENT.event_response {
                    Some(Box) => {
                        // 로그 출력
                        println!("{}", log_more_text_writer(String::from("Run response EVENT handler."), get_this_name(), LogTypeTag::INFO, format!("IP:{}", client_ip)));

                        // 이벤트 실행 결과
                        let result : Response = Box(&request);
                        // Response 생성
                        let mut response = response_parser(result);
                        // 응답 반환
                        match threadPoolArgs.tcp_stream.write_all(response.as_bytes()) {
                            Ok(_) => {}
                            Err(error) => {
                                // 로그 출력
                                println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                            }
                        }
                        // 응답 전송
                        match threadPoolArgs.tcp_stream.flush() {
                            Ok(_) => {}
                            Err(error) => {
                                // 로그 출력
                                println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                            }
                        }
                    },
                    None => {
                        // 로그 출력
                        println!("{}", log_more_text_writer(String::from("Response EVENT handler failed, no registered EVENT."), get_this_name(), LogTypeTag::INFO, format!("IP:{}", client_ip)));
                    }
                }
            }
        }
        _Error => { // 작업 실패
            // 함수 종료
            return;
        }
    }

    // 함수 종료
    return;
}