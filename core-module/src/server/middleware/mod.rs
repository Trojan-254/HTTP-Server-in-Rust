// core-module/src/server/middleware/mod.rs
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

pub mod logger;
pub mod cors;
pub mod rate_limiter;
pub mod security;

// Request and response types
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

// Type alias for middleware next function
pub type Next<'a> = Pin<Box<dyn Future<Output = Response> + Send + 'a>>;

// Middleware trait
pub trait Middleware: Send + Sync {
    fn handle<'a>(&'a self, req: Request, next: Next<'a>) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>>;
}

// Middleware chain
pub struct MiddlewareChain {
    middlewares: Vec<Arc<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        MiddlewareChain {
            middlewares: Vec::new(),
        }
    }

    pub fn add<M: Middleware + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub async fn run(&self, req: Request, handler: impl Fn(Request) -> Response + Send + Sync + 'static) -> Response {
        let handler = Arc::new(handler);
        
        // Process the middleware chain
        let mut chain = self.middlewares.iter().rev().collect::<Vec<_>>();
        
        let mut next_fn = Box::pin(async move {
            handler(req)
        }) as Pin<Box<dyn Future<Output = Response> + Send>>;
        
        // Build the middleware chain
        for middleware in chain {
            let middleware = middleware.clone();
            let prev_next = next_fn;
            
            next_fn = Box::pin(async move {
                let next = prev_next;
                middleware.handle(req, next).await
            });
        }
        
        next_fn.await
    }
}