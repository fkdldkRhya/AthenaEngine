use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::io::{BufReader, Error};
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

use crate::log::{log_text_writer, LogTypeTag};
use crate::server::request_parser::request_parser::{Request, request_parser};
use crate::server::thread_pool::thread_pool::ThreadPool;

mod thread_pool;
mod request_parser;


/// 현재 파일 정보 반환
fn get_this_name() -> String{
    return String::from("main/server");
}


/// 클라이언트 작업 성공 여부
enum TaskSuccess {
    Success,
    Error
}


/// 클라이언트 접속 이벤트
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum ClientConnectionEvent {
    CLIENT_CONNECTION,
    SayBye,
}

/// 클라이언트 접속 이벤트 Handler
pub type ClientConnectionHandlerPtr<T> = Box<dyn Fn(&T)>;

/// 클라이언트 접속 이벤트 Emitter
pub struct ClientConnectionEventEmitter<T: Hash + Eq, U> {
    handlers: HashMap<T, ClientConnectionHandlerPtr<U>>,
}

/// 클라이언트 접속 이벤트 등록 및 실행
impl<T: Hash + Eq, U> ClientConnectionEventEmitter<T, U> {
    /// Creates a new instance of `EventEmitter`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers function `handler` as a listener for `event`.  There may be
    /// multiple listeners for a single event.
    pub fn on<F>(&mut self, event: T, handler: F) where F: Fn(&U) + 'static,
    {
        self.handlers.insert(event, Box::new(handler));
    }

    /// Invokes all listeners of `event`, passing a reference to `payload` as an
    /// argument to each of them.
    pub fn emit(&self, event: T, payload: U) {
        if let Some(handler) = self.handlers.get(&event) {
            handler(&payload);
        }
    }
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
///
/// size : 생성 Thread 개수
pub fn start_server(server_ip : String, server_port : u16, thread_count : usize) {
    // TCP 리스너 생성
    let listener = TcpListener::bind(format!("{}:{}", server_ip, server_port)).unwrap();
    // Thread Pool 생성
    let pool = ThreadPool::new(thread_count);
    // TCP 연결 대기
    for stream in listener.incoming() {
        // Stream 추출
        match stream {
            Ok(stream) => { // 작업 성공!
                // Thread Pool 로 작업 전송
                pool.execute(|| {
                    handle_connection(stream);
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
fn handle_connection(mut stream: TcpStream) {
    // 작업 성공 여부
    let mut task_success = TaskSuccess::Success;

    // HTTP 요청 읽기
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result |
            match result {
                Ok(std) => std,
                Err(err) => {
                    // 오류 출력
                    println!("{}", log_text_writer(err.to_string(), get_this_name(), LogTypeTag::WARNING));
                    // 작업 실패
                    task_success = TaskSuccess::Error;
                    // 빈 문자열 반환
                    return String::from("");
                }
            }
        )
        .take_while(|line| !line.is_empty())
        .collect();

    // 작업 성공 여부 비교
    match task_success {
        _Success => { // 작업 성공
            // Request 패킷 분석
            let mut request = request_parser(&http_request);
        }
        _Error => { // 작업 실패
            // 함수 종료
            return;
        }
    }
}