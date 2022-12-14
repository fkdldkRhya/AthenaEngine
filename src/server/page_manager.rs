pub mod page_manager {
    use std::collections::HashMap;
    use std::io::Read;
    use crate::log::{log_more_text_writer, LogTypeTag};

    /// 현재 파일 정보 반환
    fn get_this_name() -> String {
        return String::from("main/server/page_manager");
    }


    /// 모든 페이지 Struct
    pub struct AllPages {
        pub(crate) pages: Option<HashMap<String, PageInfo>>
    }

    /// 페이지 정보 Struct
    pub struct PageInfo {
        pub(crate) file_path: String,
        pub(crate) is_access: bool
    }

    /// 페이지 HTML 정보 Struct
    pub struct PageFileReadInfo {
        pub(crate) value : Option<String>,
        pub(crate) is_success : IsPageFileReadSuccess
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
}