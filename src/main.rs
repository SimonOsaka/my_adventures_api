// We need this in order to compile although I'm not totally sure why.
// It has something to do with with the routes.
#![type_length_limit = "1422483"]

use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode};
#[macro_use]
extern crate log;

mod app;
mod auth;
mod errors;
mod handlers;
mod logger;
mod response;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    Logger::try_with_env()
        .unwrap()
        .log_to_file(FileSpec::default().directory("/apps/log/rust"))
        .write_mode(WriteMode::BufferAndFlush)
        .print_message()
        .duplicate_to_stdout(Duplicate::All)
        .duplicate_to_stderr(Duplicate::Warn)
        .format(crate::logger::logger_format)
        // .set_palette(("196;208;34;21;8").to_string())
        .rotate(Criterion::Age(Age::Day), Naming::Timestamps, Cleanup::KeepLogFiles(7))
        .append()
        .start()
        .unwrap();

    app::start().await;
}
