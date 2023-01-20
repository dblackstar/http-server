#![allow(dead_code)]

mod website_handle;

// from submodules is http folder
use server::Server;
use website_handle::WebsiteHandler;
use std::env;


// import server module
mod server;

// modules 
mod http;

fn main() {
    let default_path =format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Pub path = {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));   
}




