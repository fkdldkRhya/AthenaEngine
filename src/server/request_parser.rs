pub mod request_parser {
    use std::collections::HashMap;
    use urlencoding::decode;
    use crate::log::{log_text_writer, LogTypeTag};


    /// 현재 파일 정보 반환
    fn get_this_name() -> String {
        return String::from("main/server/request_parser");
    }

    /// 요청 Method
    #[derive(PartialEq)]
    pub enum Method {
        GET, POST, NOT_SUPPORTED
    }

    /// HTTP 버전
    #[derive(PartialEq)]
    pub enum HttpVersion {
        HTTP_1_0,
        HTTP_1_1,
        HTTP_2_0,
        NOT_SUPPORTED
    }

    /// Request 데이터
    pub struct Request {
        pub(crate) method : Option<Method>,
        pub(crate) target : Option<String>,
        pub(crate) host : Option<String>,
        pub(crate) http_version : Option<HttpVersion>,
        pub(crate) http_header : Option<HashMap<String, String>>,
        pub(crate) cookies: Option<HashMap<String, String>>,
        pub(crate) params: Option<HashMap<String, String>>,
        pub(crate) body: Option<String>
    }


    /// HTTP Request 요청 패킷 변환
    ///
    /// # Examples
    ///
    /// ```
    /// request_parser(http_packet)
    /// ```
    ///
    /// # Argument
    /// packet : HTTP 요청 패킷
    ///
    /// # Return
    /// Request 구조체
    pub fn request_parser(packet : &Vec<&str>) -> Request {
        // 반환 데이터 초기화
        let mut request : Request = Request {
            method: None,
            target: None,
            host: None,
            http_version: None,
            http_header: None,
            cookies: None,
            params: None,
            body: None
        };

        // Header 길이 확인
        if packet.len() >= 1 {
            // Header 추출 기본 정보
            let tag_cookie : String = String::from("Cookie: ");
            let tag_host : String = String::from("Host: ");
            // Header 추출 데이터
            let mut url : Option<String> = None;
            let mut headers_hashmap: HashMap<String, String> = HashMap::new();
            let mut cookies_hashmap: HashMap<String, String> = HashMap::new();
            let mut params_hashmap: HashMap<String, String> = HashMap::new();
            // Method, URL, HTTP Version 데이터 추출
            if packet[0].contains(" ") {
                let line1_split: Vec<&str> = packet[0].split(" ").collect();
                if line1_split.len() >= 3 {
                    request.method = Option::from(method_classify(line1_split[0]));
                    url = Option::from(String::from(line1_split[1]));
                    request.target = Option::from(String::from(line1_split[1]));
                    request.http_version = Option::from(http_version_classify(line1_split[2]));
                }
            }
            // Body 확인 변수
            let mut is_check_body_line : bool = false;
            let mut is_write_body : bool = false;
            // Body string 데이터
            let mut body_data : String = String::new();
            // Header 추출
            for line in packet {
                if is_check_body_line {
                    is_write_body = true;

                    body_data.push_str(line);
                    body_data.push_str("\r\n");
                }else {
                    // Body 구분 라인 확인
                    if line.replace(" ", "").len() == 0 {
                        // Body 구분 변수 설정
                        is_check_body_line = true;
                    }else {
                        if line.to_lowercase().contains(&tag_host.to_lowercase()) || line.to_lowercase().contains(&tag_cookie.to_lowercase()) {
                            if line.to_lowercase().contains(&tag_cookie.to_lowercase()) { // Request Header : Cookie
                                let temp_split : Vec<&str> = line.split(": ").collect();
                                let cookies : String = line.replace(&format!("{}: ", temp_split[0]), "");
                                if cookies.contains(",") {
                                    let cookies_root_split : Vec<&str> = cookies.trim().split(",").collect();
                                    for cookie in cookies_root_split {
                                        if cookie.contains("=") {
                                            let cookie_split: Vec<&str> = cookie.split("=").collect();
                                            if cookie_split.len() == 2 {
                                                cookies_hashmap.insert(cookie_split[0].to_string(), cookie_split[1].to_string());
                                            }
                                        }
                                    }
                                }
                            }else if line.to_lowercase().contains(&tag_host.to_lowercase()) { // Request Header : Host
                                let temp_split : Vec<&str> = line.split(": ").collect();
                                request.host = Option::from(line.replace(&format!("{}: ", temp_split[0]), "").trim().to_string());
                            }
                        }else {
                            if line.contains(": ") { // Request Header : Other header
                                let mut header_split : Vec<&str> = line.trim().split(": ").collect();
                                if header_split.len() >= 2 {
                                    let header_name : String = String::from(header_split[0]);
                                    let header_value : String = String::from(line.replace(&format!("{}: ", &header_name), ""));

                                    headers_hashmap.insert(header_name, header_value);
                                }
                            }
                        }
                    }
                }
            }

            // URL 파라미터 추출
            match url {
                Some(std) => {
                    let tag_params_root : String = String::from("?");
                    let tag_params_more : String = String::from("&");
                    let tag_params_value : String = String::from("=");
                    let mut url_full : String = std;
                    if url_full.contains(&tag_params_root) && url_full.contains(&tag_params_value) {
                        let mut params_full : Vec<&str> = url_full.split(&tag_params_root).collect();
                        let mut params_full : String = params_full[1].to_string();
                        if params_full.contains(&tag_params_more) {
                            let mut params_root_split : Vec<&str> = params_full.split(&tag_params_more).collect();
                            for params_ket_value in params_root_split {
                                let mut params_key_value_split : Vec<&str> = params_ket_value.split(&tag_params_value).collect();
                                if params_key_value_split.len() == 2 {
                                    let decoded = decode(params_key_value_split[1]);
                                    match decoded {
                                        Ok(cow_str) => {
                                            params_hashmap.insert(params_key_value_split[0].to_string(), cow_str.to_string());
                                        },
                                        Err(error) => {
                                            // 오류 출력
                                            println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                                        }
                                    }
                                }
                            }
                        }else if params_full.contains(&tag_params_value) {
                            let mut params_root_split : Vec<&str> = params_full.split(&tag_params_value).collect();
                            if params_root_split.len() == 2 {
                                let decoded = decode(params_root_split[1]);
                                match decoded {
                                    Ok(cow_str) => {
                                        params_hashmap.insert(params_root_split[0].to_string(), cow_str.to_string());
                                    },
                                    Err(error) => {
                                        // 오류 출력
                                        println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                                    }
                                }
                            }
                        }
                    }
                },
                _None => {}
            }

            // 데이터 입력 - Body
            if is_write_body {
                request.body = Some(body_data);
            }else {
                request.body = None;
            }

            // 데이터 입력
            request.http_header = Some(headers_hashmap);
            request.cookies = Some(cookies_hashmap);
            request.params = Some(params_hashmap);
        }

        // 로그 출력
        println!("{}", log_text_writer(String::from("Request packet analysis succeeded."), get_this_name(), LogTypeTag::INFO));

        // 데이터 반환
        return request;
    }


    /// Method 분류
    pub fn method_classify(input : &str) -> Method {
        return match input {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::NOT_SUPPORTED
        }
    }


    /// Http Version 분류
    pub fn http_version_classify(input : &str) -> HttpVersion {
        return match input {
            "HTTP/1.0" => HttpVersion::HTTP_1_0,
            "HTTP/1.1" => HttpVersion::HTTP_1_1,
            "HTTP/2" => HttpVersion::HTTP_2_0,
            _ => HttpVersion::NOT_SUPPORTED
        }
    }


    /// Http Version 원본 분류
    pub fn http_version_classify_original(input : &HttpVersion) -> String {
        return match input {
            HttpVersion::HTTP_1_0 => "HTTP/1.0".to_string(),
            HttpVersion::HTTP_1_1 => "HTTP/1.1".to_string(),
            HttpVersion::HTTP_2_0 => "HTTP/2.0".to_string(),
            _ => "HTTP/1.1".to_string()
        }
    }
}