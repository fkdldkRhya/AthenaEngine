use std::collections::HashMap;
use std::io::Read;
use crate::server::response_parser::response_parser::ResponseBody;
use crate::server::page_manager::page_manager;
use crate::server::page_manager::page_manager::{GetPageTemplateVar, page_template_parser, PageInfo};
use crate::server::response_parser::response_parser::{default_response_writer, IsResponseDataCreateSuccess, Response, ResponseCookies};

// Module - Server
mod server;
mod log;

fn main() {

    // All pages hashmap
    let mut all_page_list: HashMap<String, PageInfo> = HashMap::new();
    // 'hello.html' page setting
    let hello_page_info : PageInfo = PageInfo {
        file_path: "A:\\AthenaEngine\\Rust\\hello.html".to_string(), // HTML file path
        is_access: true // File accessibility
    };
    // '/hello.html' -> connection name
    // Insert hello page
    all_page_list.insert(String::from("/hello.html"), hello_page_info);
    // All pages list setting
    unsafe {
        page_manager::ALL_PAGES.pages = Some(all_page_list);
    }
    unsafe {
        // Request event setting
        server::EVENT.event_request = Some(Box::new(|request| {
            // Do
        }));
        // Response event setting
        server::EVENT.event_response = Some(Box::new(|request| {
            // Default response packet
            let mut response : Response = default_response_writer(&request, None, None);

            // Parse html
            match &response.body {
                None => {}
                Some(response_body) => {
                    match &response_body.body_str {
                        None => {}
                        Some(html) => { // Get default body
                            // Add variable
                            let mut var : HashMap<String, GetPageTemplateVar> = HashMap::new();
                            var.insert(String::from("variable_1"), Box::new(|| {
                                return String::from("Hello my var!");
                            }));
                            // Parsing html
                            let change_body : String = page_template_parser(html.clone(), var);
                            // Apply original response body
                            let response_body : ResponseBody = ResponseBody {
                                body_str: Some(change_body),
                            };
                            let mut new_response : Response = Response {
                                is_success: response.is_success,
                                response_code: response.response_code,
                                http_version: response.http_version,
                                headers: response.headers,
                                cookies: response.cookies,
                                body: Some(response_body),
                            };

                            return new_response;
                        }
                    }
                }
            }

            // Return response
            return response;
        }));
    }

    // Open server
    server::start_server(String::from("127.0.0.1"), 4444);
}