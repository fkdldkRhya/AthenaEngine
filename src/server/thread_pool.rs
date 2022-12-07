pub mod thread_pool {
    use std::thread;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::Mutex;
    use crate::log::{log_text_writer, LogTypeTag};

    /// 현재 파일 정보 반환
    fn get_this_name() -> String{
        return String::from("main/server/thread_pool");
    }

    enum Message {
        NewJob(Job),
        Terminate,
    }

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Message>,
    }

    trait FnBox {
        fn call_box(self: Box<Self>);
    }

    impl<F: FnOnce()> FnBox for F {
        fn call_box(self: Box<F>) {
            (*self)()
        }
    }

    type Job = Box<dyn FnBox + Send + 'static>;

    impl ThreadPool {
        /// 새 Thread Pool 생성
        ///
        /// # Examples
        ///
        /// ```
        /// let pool = ThreadPool::new(thread_count);
        /// ```
        ///
        /// # Argument
        /// size : Thread Pool 개수
        ///
        /// # Panics
        /// `new` 함수는 size 가 0일때 패닉을 일으킵니다
        pub fn new(size: usize) -> ThreadPool {
            // 0개 확인
            assert!(size > 0);
            // 변수 선언
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            // Thread Pool 생성
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool {
                workers, sender,
            }
        }

        pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            // Request task
            match self.sender.send(Message::NewJob(job)) {
                Ok(_value) => { // 성공
                    println!("{}", log_text_writer(String::from("Task execution request successful in thread pool."), get_this_name(), LogTypeTag::INFO));
                },
                Err(error) => { // 실패
                    // 오류 로그 작성
                    println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                }
            }
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            println!("{}", log_text_writer(String::from("Sending terminate message to all workers."), get_this_name(), LogTypeTag::INFO));

            for _ in &mut self.workers {
                match self.sender.send(Message::Terminate) {
                    Ok(_value) => {},
                    Err(error) => { // 실패
                        // 오류 로그 작성
                        println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                    }
                };
            }

            println!("{}", log_text_writer(String::from("Shutting down all workers."), get_this_name(), LogTypeTag::INFO));

            for worker in &mut self.workers {
                println!("{}", log_text_writer(String::from(format!("Shutting down worker {}.", worker.id)), get_this_name(), LogTypeTag::INFO));

                if let Some(thread) = worker.thread.take() {
                    match thread.join() {
                        Ok(_value) => {},
                        Err(_error) => { // 실패
                            // 오류 로그 작성
                            println!("{}", log_text_writer(String::from("Error while shutting down worker!"), get_this_name(), LogTypeTag::WARNING));
                        }
                    };
                }
            }
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

            let thread = thread::spawn(move ||{
                loop {
                    match receiver.lock() {
                        Ok(result) => {
                            match result.recv() {
                                Ok(message) => {
                                    match message {
                                        Message::NewJob(job) => {
                                            println!("{}", log_text_writer(String::from(format!("Worker {} got a job; executing.", id)), get_this_name(), LogTypeTag::INFO));

                                            job.call_box();
                                        },
                                        Message::Terminate => {
                                            println!("{}", log_text_writer(String::from(format!("Worker {} was told to terminate.", id)), get_this_name(), LogTypeTag::INFO));

                                            break;
                                        },
                                    }
                                },
                                Err(error) => { // 실패
                                    // 오류 로그 작성
                                    println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                                }
                            };
                        },
                        Err(error) => { // 실패
                            // 오류 로그 작성
                            println!("{}", log_text_writer(error.to_string(), get_this_name(), LogTypeTag::WARNING));
                        }
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
}