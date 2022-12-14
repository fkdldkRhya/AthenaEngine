extern crate core;

use std::collections::HashMap;
use crate::server::page_manager::page_manager;
use crate::server::response_parser::response_writer::{default_response_writer, Response};
use crate::server::page_manager::page_manager::{AllPages, PageInfo, read_page};
use crate::server::page_manager::page_manager::IsPageFileReadSuccess;
use crate::server::response_parser::response_writer::IsResponseDataCreateSuccess;

// Module - Server
mod server;
mod log;

fn main() {
    // 페이지 설정
    let mut list: HashMap<String, PageInfo> = HashMap::new();
    list.insert(String::from("/hello.html"), PageInfo { file_path: "A:\\AthenaEngine\\Rust\\hello.html".to_string(), is_access: true });
    unsafe { page_manager::ALL_PAGES.pages = Some(list); }

    unsafe {
        server::EVENT.event_request = Some(Box::new(|request| {

        }));

        server::EVENT.event_response = Some(Box::new(|request| {
            let response : Response = default_response_writer(&request, None, None);
            /*
            if response.is_success == IsResponseDataCreateSuccess::SUCCESS {
                match &response.headers {
                    None => {}
                    Some(value) => {
                        println!("{:?}", value);
                    }
                }
            }
             */

            return response;
        }));
    }

    server::start_server(String::from("192.168.0.25"), 2560);
}