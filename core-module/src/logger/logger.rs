use std::env;
use std::fs::File;
use std::io::Write;
use env_logger::Builder;
use log::LevelFilter;
use std::net::SocketAddr;
use std::time::Instant;


pub fn init_logger() {
   let default_log_level = "info";
   let log_file = File::create("logs/output.log").expect("Could not create log file");

   Builder::new()
       .format(|buf, record| {
          writeln!(
            buf,
            "{} [{}] - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
          )
       })
       .filter(None, LevelFilter::Info)
       .target(env_logger::Target::Pipe(Box::new(log_file)))
       .try_init();
   // Override the log level if the RUST_LOG environment variable is set
   if let Ok(log_level) = env::var("RUST_LOG") {
      Builder::from_default_env()
          .filter(None, LevelFilter::Info)
          .try_init();
   } else {
     // Use default log level
     env::set_var("RUST_LOG", default_log_level);
   }
}


pub fn init_logger() {
   let default_log_level = "info";
   let log_file = File::create("logs/output.log").expect("Failed to create a log file");

   let mut builder = env_logger::builder();
   builder.add_target(log::Target::File, log_file).filter(
      None, log::LevelFilter::Info
   ).format(|buf, record| {
      writeln!(buf, "{} [{}] - {}", )
   }).init();


}