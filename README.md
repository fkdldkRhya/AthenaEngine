[![crates.io](https://img.shields.io/crates/v/httparse.svg)](https://crates.io/crates/AthenaEngine)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)

# AthenaEngine
Web server engine for rust

## External library
```
[dependencies]
chrono = "0.4.23"
urlencoding = "2.1.2"
```

## How to use?
```Rust
use std::collections::HashMap;
use AthenaEngine::server;
use AthenaEngine::server::page_manager::page_manager;
use AthenaEngine::server::page_manager::page_manager::PageInfo;
use AthenaEngine::server::response_parser::response_parser::{default_response_writer, IsResponseDataCreateSuccess, Response, ResponseCookies};

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
    
    
    // Client connection event setting
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
```

# Expansion module description
## server::page_manager::page_template_parser (function) 
> 
> You can call the Body structure and template functions. 
> 
> < Rust file >
> ```Rust
> // Response event setting
> server::EVENT.event_response = Some(Box::new(|request| {
>   // Default response packet
>   let mut response : Response = default_response_writer(&request, None, None);
> 
>   // Parse html
>   match &response.body {
>       None => {}
>       Some(response_body) => {
>           match &response_body.body_str {
>               None => {}
>               Some(html) => { // Get default body
>                   // Add variable
>                   let mut var : HashMap<String, GetPageTemplateVar> = HashMap::new();
>                   var.insert(String::from("variable_1"), Box::new(|| {
>                       return String::from("Hello my var!");
>                   }));
>                   // Parsing html
>                   let change_body : String = page_template_parser(html.clone(), var);
>                   // Apply original response body
>                   let response_body : ResponseBody = ResponseBody {
>                       body_str: Some(change_body),
>                   };
>                   let mut new_response : Response = Response {
>                       is_success: response.is_success,
>                       response_code: response.response_code,
>                       http_version: response.http_version,
>                       headers: response.headers,
>                       cookies: response.cookies,
>                       body: Some(response_body),
>                   };
> 
>                   return new_response;
>               }
>           }
>       }
>   }
> 
>   // Return response
>   return response;
> }));
> ```
>
> < HTML file >
> ```
> <!DOCTYPE html>
> <html lang="en">
> <head>
>   <meta charset="UTF-8">
>   <title>Title</title>
> </head>
> <body>
> <#>var.variable_1
> </body>
> </html>
> ```



# License
MIT License

Copyright (c) 2022 CHOI SI-HUN

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
