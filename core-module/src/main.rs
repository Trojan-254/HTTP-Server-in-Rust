use std::net::TcpListener;
use core_module::threadpool::ThreadPool;
use core_module::server::handle_connection;
use log::info;

mod logger;

fn main() {
    // env_logger::init();
    logger::init_logger();
    let listener = TcpListener::bind("127.0.0.1:4228").unwrap();
    info!("Rusty HTTP Server is up and running...");
    log::info!("Perfoming Health checks!.....");
    log::info!("Perfomance. Optimized.....");
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(100) {
        let stream = stream.unwrap();
        
        pool.execute(|| { 
            // println!("Handling connection...");
            info!("Connection established!");
            // println!("Stream: {:?}", stream);
            // println!("Connection established!...HTTP Server listening on port 4228..");
            handle_connection(stream);
        });
        // pool.execute(|| {
        //     println!("Handling connection...");
        //     match handle_connection(stream) {
        //         Ok(_) => {
        //             println!("Connection established!...HTTP Server listening on port 4228..");
        //         }
        //         Err(e) => {
        //             println!("Failed to handle connection: {:?}", e);
        //         }
        //     }
        // });
    }
    
    println!("Shutting down.");
}

