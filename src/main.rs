// We need this in order to compile although I'm not totally sure why.
// It has something to do with with the routes.
#![type_length_limit = "1422483"]

use flexi_logger::{Age, Cleanup, Criterion, Duplicate, LogTarget, Logger, Naming};
#[macro_use]
extern crate log;

mod app;
mod auth;
mod errors;
mod handlers;
mod response;
mod routes;
mod logger;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    Logger::with_env()
        .log_target(LogTarget::File)
        .log_to_file()
        .directory("/apps/log/rust")
        .print_message()
        .duplicate_to_stdout(Duplicate::All)
        .duplicate_to_stderr(Duplicate::Warn)
        .format(crate::logger::logger_format)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7),
        )
        .append()
        .start()
        .unwrap();

    app::start().await;
}
