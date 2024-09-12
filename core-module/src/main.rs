use std::net::TcpListener;
use core_module::threadpool::ThreadPool;
use core_module::server::handle_connection;
use log::{info, error};


mod logger;

fn main() {
    // env_logger::init();
    logger::init_logger();
    let listener = TcpListener::bind("0.0.0.1:4228").unwrap();
    info!("Rusty HTTP Server is up and running...");
    log::info!("Perfoming Health checks!.....");
    log::info!("Perfomance. Optimized.....");
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(100) {
        let stream = match stream {
           Ok(stream) => stream,
           Err(e) => {
              error!("Failed to accept connection: {:?}", e);
              continue;
           }
        };
        
        pool.execute(move || { 
            if let Err(e) = handle_connection(stream) {
               error!("Failed to handle connection: {:?}", e);
            } else {
              info!("Connection established. Handling connection...");
            }
        });
    }

}

