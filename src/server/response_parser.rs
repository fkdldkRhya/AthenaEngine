pub mod response_parser {
    use std::collections::HashMap;
    use std::fmt::format;
    use std::io::Bytes;
    use chrono::{Datelike, DateTime, Timelike, Utc};
    use crate::log::{log_text_writer, LogTypeTag};
    use crate::server::request_parser::request_parser::{http_version_classify_original, HttpVersion, Method, Request};
    use crate::server::page_manager::page_manager::{PageFileReadInfo, read_page};
    use crate::server::page_manager::page_manager::IsPageFileReadSuccess;


    /// 현재 파일 정보 반환
    fn get_this_name() -> String {
        return String::from("main/server/response_parser");
    }


    /// 서버 이름 반환
    fn get_server_name() -> String { return String::from("Athena-Engine") }


    /// Response 데이터
    pub struct Response {
        pub is_success: IsResponseDataCreateSuccess,
        pub response_code: Option<HttpStateCode>,
        pub http_version: Option<HttpVersion>,
        pub headers: Option<HashMap<String, String>>,
        pub cookies: Option<Vec<ResponseCookies>>,
        pub body: Option<ResponseBody>
    }

    /// Response 쿠키 데이터
    pub struct ResponseCookies {
        pub name : String,
        pub value : String,
        pub path : String
    }

    /// Response body 데이터
    pub struct ResponseBody {
        pub body_str: Option<String>
    }

    /// 페이지 HTML 정보 불러오기 작업 성공 여부 Enum
    #[derive(PartialEq)]
    pub enum IsResponseDataCreateSuccess {
        SUCCESS, FAIL
    }

    /// HTTP 상태 응답 코드
    #[derive(PartialEq)]
    pub enum HttpStateCode {
        HTTP_110,
        HTTP_111,

        HTTP_200,
        HTTP_300,
        HTTP_301,
        HTTP_302,
        HTTP_303,
        HTTP_304,
        HTTP_307,
        HTTP_308,
        HTTP_310,

        HTTP_400,
        HTTP_401,
        HTTP_402,
        HTTP_403,
        HTTP_404,
        HTTP_405,
        HTTP_406,
        HTTP_407,
        HTTP_408,
        HTTP_409,
        HTTP_410,
        HTTP_411,
        HTTP_412,
        HTTP_413,
        HTTP_414,
        HTTP_415,
        HTTP_416,
        HTTP_417,
        HTTP_418,
        HTTP_420,
        HTTP_422,
        HTTP_423,
        HTTP_424,
        HTTP_425,
        HTTP_426,
        HTTP_428,
        HTTP_429,
        HTTP_431,

        HTTP_500,
    }
    

    /// HTTP 상태 변환기
    pub fn default_http_state_writer(http_code : &HttpStateCode) -> &'static str {
        return match http_code {
            HttpStateCode::HTTP_110 => "110 Connection Timed Out",
            HttpStateCode::HTTP_111 => "111 Connection refused",
            HttpStateCode::HTTP_200 => "200 OK",
            HttpStateCode::HTTP_300 => "300 Multiple Choice",
            HttpStateCode::HTTP_303 => "303 See Other",
            HttpStateCode::HTTP_304 => "304 Not Modified",
            HttpStateCode::HTTP_307 => "307 Temporary Redirect",
            HttpStateCode::HTTP_308 => "308 Permanent Redirect",
            HttpStateCode::HTTP_301 => "301 Moved Permanently",
            HttpStateCode::HTTP_302 => "302 Found",
            HttpStateCode::HTTP_310 => "310 Too many redirects",
            HttpStateCode::HTTP_400 => "400 Bad Request",
            HttpStateCode::HTTP_404 => "404 Not Found",
            HttpStateCode::HTTP_500 => "500 Internal Server Error",
            HttpStateCode::HTTP_401 => "401 Unauthorized",
            HttpStateCode::HTTP_402 => "402 Payment Required",
            HttpStateCode::HTTP_403 => "403 Forbidden",
            HttpStateCode::HTTP_405 => "405 Method Not Allowed",
            HttpStateCode::HTTP_406 => "406 Not Acceptable",
            HttpStateCode::HTTP_407 => "407 Proxy Authentication Required",
            HttpStateCode::HTTP_408 => "408 Request Timeout",
            HttpStateCode::HTTP_409 => "409 Conflict",
            HttpStateCode::HTTP_410 => "410 Gone",
            HttpStateCode::HTTP_411 => "411 Length Required",
            HttpStateCode::HTTP_412 => "412 Precondition Failed",
            HttpStateCode::HTTP_413 => "413 Request Entity Too Large",
            HttpStateCode::HTTP_414 => "414 Request-URI Too Long",
            HttpStateCode::HTTP_415 => "415 Unsupported Media Type",
            HttpStateCode::HTTP_416 => "Requested Range Not Satisfiable",
            HttpStateCode::HTTP_417 => "Expectation Failed",
            HttpStateCode::HTTP_418 => "I'm a teapot (RFC 2324)",
            HttpStateCode::HTTP_420 => "Enhance Your Calm (Twitter)",
            HttpStateCode::HTTP_422 => "422 Unprocessable Entity (WebDAV)",
            HttpStateCode::HTTP_423 => "423 Locked (WebDAV)",
            HttpStateCode::HTTP_424 => "424 Failed Dependency (WebDAV)",
            HttpStateCode::HTTP_425 => "425 Reserved for WebDAV",
            HttpStateCode::HTTP_426 => "426 Upgrade Required",
            HttpStateCode::HTTP_428 => "428 Precondition Required",
            HttpStateCode::HTTP_429 => "429 Too Many Requests",
            HttpStateCode::HTTP_431 => "431 Request Header Fields Too Large",
        };
    }


    /// 기본 응답 Body 생성기
    pub fn default_body_writer(http_code : &HttpStateCode) -> String {
        let mut template = String::from("<head><title>#Result#</title><body>#Result#</body></head>");
        let replace_tag = String::from("#Result#");

        template = template.replace(&replace_tag, default_http_state_writer(http_code));

        return template;
    }


    /// 기본 응답 Header 생성기
    pub fn default_response_header_writer() -> HashMap<String, String> {
        let mut header : HashMap<String, String> = HashMap::new();

        // 헤더 데이터 설정 - Date
        let header_setting_date: DateTime<Utc> = Utc::now();
        let header_setting_date_month_to_str : &str = match header_setting_date.month() {
            1 => "Jun",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Jun"
        };
        header.insert(String::from("Date"),
                      String::from(format!("{}, {:0>2} {} {} {:0>2}:{:0>2}:{:0>2} GMT",
                                           header_setting_date.weekday(),
                                           header_setting_date.day(),
                                           header_setting_date_month_to_str,
                                           header_setting_date.year(),
                                           header_setting_date.hour(),
                                           header_setting_date.minute(),
                                           header_setting_date.second())));
        // 헤더 데이터 설정 - Server
        header.insert(String::from("Server"), get_server_name());
        // 헤더 데이터 설정 - Connection
        header.insert(String::from("Connection"), String::from("close"));
        // 헤더 데이터 설정 - Connection
        header.insert(String::from("Pragma"), String::from("no-cache"));
        // 헤더 데이터 설정 - Content-Type
        header.insert(String::from("Content-Type"), String::from("text/html; charset=UTF-8"));
        // 헤더 데이터 설정 - Content-Language
        header.insert(String::from("Content-Language"), String::from("ko-KR"));
        // 헤더 데이터 설정 - Access-Control-Allow-Origin
        header.insert(String::from("Access-Control-Allow-Origin"), String::from("*"));

        return header;
    }



    /// HTTP Response 응답 패킷 데이터 생성
    ///
    /// # Examples
    ///
    /// ```
    /// default_response_writer(&request, None, None)
    /// ```
    ///
    /// # Argument
    /// request : HTTP 응답 데이터
    ///
    /// cookies : 응답 헤더에 추가할 쿠키
    ///
    /// input_header : 응답 헤더 추가 작성
    ///
    /// # Return
    /// Response 구조체
    pub fn default_response_writer(request : &Request, cookies : Option<Vec<ResponseCookies>>, input_header : Option<HashMap<String, String>>) -> Response {
        // 반환 데이터 초기화
        let mut response : Response = Response {
            is_success: IsResponseDataCreateSuccess::SUCCESS,
            response_code: None,
            http_version: None,
            headers: None,
            cookies: None,
            body: None
        };

        // 데이터 생성 - HTTP 응답 코드
        let mut response_http_version : HttpVersion = HttpVersion::HTTP_1_1;
        // 데이터 생성 - HTTP 응답 코드
        let mut response_code : HttpStateCode = HttpStateCode::HTTP_200;
        // 데이터 생성 - HTTP 응답 Body
        let mut response_body : ResponseBody = ResponseBody {
            body_str: None
        };

        // Method 데이터 추출
        match &request.method {
            Some(method) => {
                // Method 지원 여부 확인
                if method != &Method::NOT_SUPPORTED {
                    // 데이터 추출 - HTTP 버전
                    match &request.http_version {
                        Some(http_version) => {
                            // HTTP 버전 지원 여부 확인
                            if http_version != &HttpVersion::NOT_SUPPORTED {
                                // 데이터 삽입 - HTTP Version
                                match &request.http_version {
                                    None => {}
                                    Some(version) => {
                                        response_http_version = match version {
                                            HttpVersion::HTTP_1_0 => HttpVersion::HTTP_1_0,
                                            HttpVersion::HTTP_1_1 => HttpVersion::HTTP_1_1,
                                            HttpVersion::HTTP_2_0 => HttpVersion::HTTP_2_0,
                                            HttpVersion::NOT_SUPPORTED => HttpVersion::HTTP_1_1
                                        };
                                    }
                                }

                                // 데이터 삽입 - Cookies
                                response.cookies = cookies;

                                // 요청 페이지 읽기
                                match &request.target {
                                    Some(request_page) => {
                                        // 페이지 정보 불러오기
                                        let page_read_data : PageFileReadInfo = read_page(String::from(request_page));
                                        // 성공 여부 확인
                                        if page_read_data.is_success == IsPageFileReadSuccess::SUCCESS { // 페이지 읽기 성공
                                            response_code = HttpStateCode::HTTP_200;
                                            response_body.body_str = page_read_data.value;
                                        }else if page_read_data.is_success == IsPageFileReadSuccess::FAIL { // 400 오류 발생
                                            response_code = HttpStateCode::HTTP_400;
                                            response_body.body_str = Some(default_body_writer(&response_code));
                                        }else { // 404 오류 발생
                                            response_code = HttpStateCode::HTTP_404;
                                            response_body.body_str = Some(default_body_writer(&response_code))
                                        }
                                    },
                                    None => { // 404 오류 발생
                                        response_code = HttpStateCode::HTTP_404;
                                        response_body.body_str = Some(default_body_writer(&response_code));
                                    }
                                }
                            }else { // 426 오류 발생
                                response.is_success = IsResponseDataCreateSuccess::FAIL;
                            }
                        },
                        None => { // 426 오류 발생
                            response.is_success = IsResponseDataCreateSuccess::FAIL;
                        }
                    };
                }else { // 426 오류 발생
                    response.is_success = IsResponseDataCreateSuccess::FAIL;
                }
            }
            None => { // 426 오류 발생
                response.is_success = IsResponseDataCreateSuccess::FAIL;
            }
        }

        // 헤더 데이터 HashMap
        let mut header : HashMap<String, String> = default_response_header_writer();

        // 헤더 데이터 추가 - Content-Disposition
        header.insert(String::from("Content-Disposition"), String::from("inline"));
        // 헤더 데이터 추가 - Cache-Control
        header.insert(String::from("Cache-Control"), String::from("no-cache"));
        // Header 추가
        match input_header {
            Some(input) => {
                for (key, value) in input {
                    // 헤더 데이터 추가
                    header.insert(key, value);
                }
            },
            None => {}
        }

        // 헤더 데이터 추가 - Content-Length
        match &response_body.body_str {
            Some(body) => {
                header.insert(String::from("Content-Length"), body.len().to_string());
                header.insert(String::from("Accept-Ranges"), String::from("bytes"));
            },
            None => {
                header.insert(String::from("Content-Length"), String::from("0"));
            }
        };

        // Body 설정
        response.body = Some(response_body);

        // Response 데이터 설정
        response.response_code = Some(response_code);
        response.http_version = Some(response_http_version);
        response.headers = Some(header);

        // 로그 출력
        println!("{}", log_text_writer(String::from("Default response packet creation succeeded."), get_this_name(), LogTypeTag::INFO));

        // 데이터 반환
        return response;
    }


    /// 응답 Struct 를 String 형식으로 변환
    pub fn response_parser(response : Response) -> String {
        // 기본 Response
        let default_response = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nDate: Wed, 14 Dec 2022 00:25:57 GMT\r\nAccess-Control-Allow-Origin: *\r\nContent-Disposition: inline\r\nContent-Language: ko-KR");
        // Response 생성
        let mut response_str = String::new();
        if response.is_success == IsResponseDataCreateSuccess::SUCCESS {
            match &response.http_version {
                Some(http_version) => {
                    match &response.response_code {
                        Some(response_code) => {
                            match &response.headers {
                                Some(response_header) => {
                                    let mut header : String = String::new();
                                    for (response_header_key, response_header_value) in response_header {
                                        let line : String = format!("{}: {}\r\n", response_header_key, response_header_value);
                                        header.push_str(&line);
                                    }

                                    // 쿠키 설정
                                    match &response.cookies {
                                        Some(cookies) => {
                                            for cookie in cookies {
                                                let line = String::from(format!("Set-Cookie: {}={}; Path={}\r\n", cookie.name, cookie.value, cookie.path));
                                                header.push_str(&line);
                                            }
                                        },
                                        None => {}
                                    }

                                    response_str = response_format(http_version, response_code, header, response.body);
                                }
                                None => {
                                    return default_response;
                                }
                            }
                        },
                        None => {
                            return default_response;
                        }
                    }
                },
                None => {
                    return default_response;
                }
            }
        }else {
            return default_response;
        }

        // 데이터 반환
        return response_str;
    }


    /// Response 문자열 형식으로 변환
    fn response_format(http_version : &HttpVersion, response_code : &HttpStateCode, header_str : String, body : Option<ResponseBody>) -> String {
        return match body {
            Some(body) => {
                match &body.body_str {
                    Some(body) => {
                        String::from(
                            format!("{} {}\r\n{}\r\n{}",
                                    http_version_classify_original(http_version),
                                    default_http_state_writer(response_code),
                                    header_str,
                                    body))
                    },
                    None => {
                        String::from(
                            format!("{} {}\r\n{}",
                                    http_version_classify_original(http_version),
                                    default_http_state_writer(response_code),
                                    header_str))
                    }
                }
            }
            None => {
                String::from(
                    format!("{} {}\r\n{}",
                            http_version_classify_original(http_version),
                            default_http_state_writer(response_code),
                            header_str))
            }
        }
    }
}