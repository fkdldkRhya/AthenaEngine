# AthenaEngine
Web server template engine for rust

### How to use ?
'''
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
            // Cookies setting
            let mut cookies_list : Vec<ResponseCookies> = Vec::new();
            let cookie : ResponseCookies = ResponseCookies {
                name: "test-my-cookie".to_string(),
                value: "test-my-cookie-value".to_string(),
                path: "/".to_string(),
            };
            cookies_list.push(cookie);
            // Default response packet
            let response : Response = default_response_writer(&request, Some(cookies_list), None);

            // Get response value
            if response.is_success == IsResponseDataCreateSuccess::SUCCESS {
                match &response.headers {
                    None => {}
                    Some(value) => {
                        // Print all headers
                        println!("{:?}", value);
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
'''
