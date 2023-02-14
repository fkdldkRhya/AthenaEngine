pub mod page_manager {
    use std::collections::HashMap;
    use std::io::Read;
    use crate::log::{log_more_text_writer, log_text_writer, LogTypeTag};

    /// 현재 파일 정보 반환
    fn get_this_name() -> String {
        return String::from("main/server/page_manager");
    }


    /// 변수 반환 함수 지정
    pub type GetPageTemplateVar = Box<dyn Send + (Fn() -> String) + 'static>;

    /// 모든 페이지 Struct
    pub struct AllPages {
        pub pages: Option<HashMap<String, PageInfo>>
    }

    /// 페이지 정보 Struct
    pub struct PageInfo {
        pub file_path: String,
        pub is_access: bool
    }

    /// 페이지 HTML 정보 Struct
    pub struct PageFileReadInfo {
        pub value : Option<String>,
        pub is_success : IsPageFileReadSuccess
    }

    /// 페이지 HTML 정보 불러오기 작업 성공 여부 Enum
    #[derive(PartialEq)]
    pub enum IsPageFileReadSuccess {
        SUCCESS, FAIL, NO_DATA
    }

    /// 페이지 리스트
    /// 주의: 페이지 추가시 경로는 모두 소문자로 입력
    pub static mut ALL_PAGES : AllPages = AllPages {
        pages: None
    };


    /// HTML 파일 Reader
    ///
    /// # Examples
    ///
    /// ```
    /// read_page(String::from("/hello.html"))
    /// ```
    ///
    /// # Argument
    /// page_path : HTTP 경로
    ///
    /// all_pages : 모든 페이지 정보
    ///
    /// # Return
    /// PageFileReadInfo 구조체
    pub fn read_page(page_path : String) -> PageFileReadInfo {
        let mut read_result : PageFileReadInfo = PageFileReadInfo {
            value: None,
            is_success: IsPageFileReadSuccess::NO_DATA,
        };

        unsafe {
            match &ALL_PAGES.pages {
                Some(map) => {
                    if map.contains_key(&page_path.to_lowercase()) {
                        let page_info = map.get(&page_path.to_lowercase());
                        match page_info {
                            Some(page_info) => {
                                if page_info.is_access {
                                    let mut read_value = std::fs::File::open(&page_info.file_path);
                                    match read_value {
                                        Ok(mut value) => {
                                            let mut contents = String::new();
                                            match value.read_to_string(&mut contents) {
                                                Ok(_) => {
                                                    read_result.value = Some(contents);
                                                    read_result.is_success = IsPageFileReadSuccess::SUCCESS;
                                                }
                                                Err(error) => {
                                                    read_result.is_success = IsPageFileReadSuccess::FAIL;

                                                    // 로그 출력
                                                    println!("{}", log_more_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING, String::from(&page_info.file_path)));
                                                }
                                            }
                                        },
                                        Err(error) => {
                                            read_result.is_success = IsPageFileReadSuccess::FAIL;

                                            // 로그 출력
                                            println!("{}", log_more_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING, String::from(&page_info.file_path)));
                                        }
                                    }
                                }else {
                                    read_result.is_success = IsPageFileReadSuccess::FAIL;
                                }
                            },
                            None => {
                                read_result.is_success = IsPageFileReadSuccess::NO_DATA;
                            }
                        }
                    }
                },
                None => {
                    read_result.is_success = IsPageFileReadSuccess::NO_DATA;
                }
            }
        }

        return read_result;
    }


    /// Page template parser
    pub fn page_template_parser(html : String, var : HashMap<String, GetPageTemplateVar>) -> String {
        // 기본 Tag
        let tag_root = String::from("<#>");
        let tag_var = String::from(format!("{}var", tag_root));
        let tag_control = String::from(format!("{}control", tag_root));
        // 제어문 Tag
        let tag_control_for = String::from(format!("{}.for", tag_control));
        let tag_control_for_end = String::from(format!("{}.for_end", tag_control));

        // HTML 반환
        return if html.contains(&tag_root) && html.contains("\n") {
            let mut new_html = String::from(html.clone());
            // 변수 치환
            for (var_name, fun_value) in var {
                let value : String = fun_value();

                new_html = new_html.replace(&String::from(format!("{}.{}", &tag_var, var_name)), &value);
            }
            // 오류 확인
            if new_html.contains(&tag_var) {
                // 로그 출력
                print_html_parse_error();

                return html;
            }

            // 제어문 컴파일
            let mut control_new_html = String::from(&new_html);
            let mut control_split = control_new_html.split("\r\n");
            let mut control_split : Vec<&str> = control_split.collect();
            let mut control_for_enable : bool = false;
            let mut control_for_replace : String = String::new();
            let mut control_for_text : String = String::new();
            let mut control_for_new_text : String = String::new();
            let mut control_for_start : usize = 0;
            let mut control_for_end : usize = 0;
            for control_line in control_split {
                // For 문 확인
                if control_line.contains(&tag_control_for) && !control_line.contains(&tag_control_for_end) {
                    // 변수 초기화
                    control_for_text.clear();
                    control_for_new_text.clear();
                    control_for_replace.clear();
                    control_for_start = 0;
                    control_for_end = 0;
                    // For 문 분석
                    let data = control_line.clone().replace(&tag_control_for, "").replace(" ", "");
                    // 오류 확인
                    if !data.contains(",") {
                        // 로그 출력
                        print_html_parse_error();

                        return html;
                    }
                    // For 문 데이터
                    let data : Vec<&str> = data.split(",").collect();
                    if data.len() >= 2 {
                        control_for_start = match data[0].parse() {
                            Ok(value) => value,
                            Err(_) => 0
                        };
                        control_for_end = match data[1].parse() {
                            Ok(value) => value,
                            Err(_) => 0
                        };
                        control_for_replace.push_str(&String::from(format!("{}\r\n", &control_line)));
                        control_for_enable = true;
                    }else {
                        control_for_enable = false;
                    }
                }
                // For 문 내용 확인
                if control_for_enable && !control_line.contains(&tag_control_for) && !control_line.contains(&tag_control_for_end) {
                    control_for_text.push_str(&control_line);
                    control_for_replace.push_str(&String::from(format!("{}\r\n", &control_line)));
                }
                // For 문 종료 확인
                if control_line.contains(&tag_control_for_end) && control_for_enable {
                    control_for_enable = false;
                    control_for_replace.push_str(&String::from(format!("{}\r\n", &control_line)));

                    for _ in control_for_start..control_for_end {
                        control_for_new_text.push_str(&String::from(format!("{}\r\n", &control_for_text)));
                    }
                    new_html = new_html.replace(&control_for_replace, &control_for_new_text);
                }
            }

            new_html
        }else {
            html
        }
    }



    /// HTML 파싱 오류 출력
    fn print_html_parse_error() {
        // 로그 출력
        println!("{}", log_text_writer(String::from("Syntax error, unknown error while parsing HTML from engine."), get_this_name(), LogTypeTag::WARNING));
    }
}