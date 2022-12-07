use crate::log::log_text_writer;
use crate::log::LogTypeTag::INFO;

// Module - Server
mod server;
mod log;

fn main() {
    server::start_server(String::from("127.0.0.0"), 8080, 4);
}