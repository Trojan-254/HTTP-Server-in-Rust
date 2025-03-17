/// Logging configuration logic.

use env_logger::{Builder, Env};
use log::LevelFilter;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct LogConfig {
    pub stdout: bool,
    pub file_output: bool,
    pub separate_error_log: bool,
    pub log_level: LevelFilter,
    pub log_dir: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            stdout: true,
            file_output: true,
            separate_error_log: true,
            log_level: LevelFilter::Info,
            log_dir: "logs".to_string(),
        }
    }
}

pub fn init_logger() {
    init_logger_with_config(LogConfig::default())
}

pub fn init_logger_with_config(config: LogConfig) {
    // Set up the environment variable for logging levels
    let env = Env::default().filter_or("RUST_LOG", config.log_level.to_string());
    
    // Create the log directory if it doesn't exist
    if config.file_output {
        if let Err(e) = fs::create_dir_all(&config.log_dir) {
            eprintln!("Failed to create log directory '{}': {}", config.log_dir, e);
            return;
        }
    }
    
    // Prepare log files if file logging is enabled
    let log_file = if config.file_output {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(format!("{}/output.log", config.log_dir))
        {
            Ok(file) => Some(file),
            Err(e) => {
                eprintln!("Failed to open output log: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    let error_log_file = if config.file_output && config.separate_error_log {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(format!("{}/error.log", config.log_dir))
        {
            Ok(file) => Some(file),
            Err(e) => {
                eprintln!("Failed to open error log file: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Build and initialize the logger
    let mut builder = Builder::from_env(env);
    
    builder.format(move |buf, record| {
        // Build the log message
        let log_message = format!(
            "{} [{}] - {}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        );
        
        // Write to stdout if enabled
        if config.stdout {
            if let Err(e) = buf.write_all(log_message.as_bytes()) {
                eprintln!("Failed to write log message to stdout: {}", e);
            }
        }
        
        // Write to log files if enabled
        if let Some(ref file) = log_file {
            if let Some(mut file_clone) = file.try_clone().ok() {
                if record.level() != log::Level::Error || error_log_file.is_none() {
                    if let Err(e) = file_clone.write_all(log_message.as_bytes()) {
                        eprintln!("Failed to write to output log file: {}", e);
                    }
                }
            }
        }
        
        // Write errors to separate error log if enabled
        if record.level() == log::Level::Error {
            if let Some(ref error_file) = error_log_file {
                if let Some(mut error_file_clone) = error_file.try_clone().ok() {
                    if let Err(e) = error_file_clone.write_all(log_message.as_bytes()) {
                        eprintln!("Failed to write to error log file: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    });
    
    // Initialize the logger
    if let Err(e) = builder.try_init() {
        eprintln!("Failed to initialize logger: {}", e);
    }
}