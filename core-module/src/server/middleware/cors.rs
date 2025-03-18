// core-module/src/server/middleware/cors.rs
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::{Middleware, Request, Response, Next};

pub struct CorsConfig {
    pub allow_origins: Vec<String>,
    pub allow_methods: Vec<String>,
    pub allow_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: Option<u32>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        CorsConfig {
            allow_origins: vec!["*".to_string()],
            allow_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), 
                               "DELETE".to_string(), "OPTIONS".to_string()],
            allow_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            allow_credentials: true,
            max_age: Some(86400), // 24 hours
        }
    }
}

pub struct Cors {
    config: CorsConfig,
}

impl Cors {
    pub fn new(config: CorsConfig) -> Self {
        Cors { config }
    }
    
    pub fn default() -> Self {
        Cors::new(CorsConfig::default())
    }
}

impl Middleware for Cors {
    fn handle<'a>(&'a self, req: Request, next: Next<'a>) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(async move {
            // Handle preflight OPTIONS request
            if req.method == "OPTIONS" {
                let mut response = Response {
                    status: 204,
                    headers: Vec::new(),
                    body: Vec::new(),
                };
                
                // Add CORS headers
                let origins = self.config.allow_origins.join(", ");
                response.headers.push(("Access-Control-Allow-Origin".to_string(), origins));
                
                let methods = self.config.allow_methods.join(", ");
                response.headers.push(("Access-Control-Allow-Methods".to_string(), methods));
                
                let headers = self.config.allow_headers.join(", ");
                response.headers.push(("Access-Control-Allow-Headers".to_string(), headers));
                
                if self.config.allow_credentials {
                    response.headers.push(("Access-Control-Allow-Credentials".to_string(), "true".to_string()));
                }
                
                if let Some(max_age) = self.config.max_age {
                    response.headers.push(("Access-Control-Max-Age".to_string(), max_age.to_string()));
                }
                
                return response;
            }
            
            // For non-OPTIONS requests, process the request and add CORS headers to the response
            let mut response = next.await;
            
            // Add CORS headers to the response
            let origins = self.config.allow_origins.join(", ");
            response.headers.push(("Access-Control-Allow-Origin".to_string(), origins));
            
            if self.config.allow_credentials {
                response.headers.push(("Access-Control-Allow-Credentials".to_string(), "true".to_string()));
            }
            
            response
        })
    }
}