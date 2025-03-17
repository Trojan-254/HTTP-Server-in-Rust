// core-module/src/logger/mod.rs
mod config;


pub mod config::LogConfig;
pub use config::init_logger;
