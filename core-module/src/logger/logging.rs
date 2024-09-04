use env_logger::{Builder, Env};
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logger() {
    // Set up of the logger that logs 
    // to the stdout and file

    let env = Env::default().filter_or("RUST_LOG", "info");

    let log_file = OpenOptions::new()
         .create(true)
         .write(true)
         .append(true)
         .open("./logs/output.log")
         .unwrap();

    let error_log_file = OpenOptions::new()
          .create(true)
          .write(true)
          .append(true)
          .open("./logs/error.log")
          .unwrap();

    Builder::from_env(env)
        .target(env_logger::Target::Stdout) // Logs to std out
        .format(move |buf, record| {
            let mut log_file = log_file.try_clone().unwrap();
            let mut error_log_file = error_log_file.try_clone().unwrap();
            let log_message = format!(
                "{} [{}] - {}\n",
                chrono::Local::now().format("%Y-%m-%d-%H-%M:%S"),
                record.level(),
                record.args()
            );

            buf.write_all(log_message.as_bytes()).unwrap();

            if record.level() == log::Level::Error {
                error_log_file.write_all(log_message.as_bytes()).unwrap();
            } else {
                log_file.write_all(log_message.as_bytes()).unwrap();
            }
            Ok(())
        })
        .init();
}