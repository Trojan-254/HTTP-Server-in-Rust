use env_logger::{Builder, Env};
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

pub fn init_logger() {
    // Set up the environment variable for logging levels
    let env = Env::default().filter_or("RUST_LOG", "info");

    // Ensure the logs direcory exists
    let log_dir = "logs";
    if let Err(e) = fs::create_dir_all(log_dir) {
       eprintln!("Failed to create log directory '{}': {}", log_dir, e);
       return;
    }

    // Open the log files
    let log_file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}/output.log", log_dir))
    {
        Ok(file) => file,
        Err(e) => {
           eprintln!("Failed to open output log: {}", e);
           return;
        }
    };

    let error_log_file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}/error.log", log_dir))
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open error log file: {}", e);
            return;
        }
    };

    // Initialize the logger
    Builder::from_env(env)
    .target(env_logger::Target::Stdout) // Logs to stdout
    .format(move |buf, record| {
        // Attempt to clone the log files
        let mut log_file = match log_file.try_clone() {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to clone log file handle: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };
        let mut error_log_file = match error_log_file.try_clone() {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to clone error log file handle: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };

        // Build the log message
        let log_message = format!(
            "{} [{}] - {}\n",
            chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"),
            record.level(),
            record.args()
        );

        // Write to stdout
        if buf.write_all(log_message.as_bytes()).is_err() {
            eprintln!("Failed to write log message to stdout");
        }

        // Write to the appropriate log files
        if record.level() == log::Level::Error {
            if let Err(e) = error_log_file.write_all(log_message.as_bytes()) {
                eprintln!("Failed to write to error log file: {}", e);
            }
        } else {
            if let Err(e) = log_file.write_all(log_message.as_bytes()) {
                eprintln!("Failed to write to output log file: {}", e);
            }
        }

        Ok(())
    })
    .init();

}
