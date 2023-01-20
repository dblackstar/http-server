// tcp module to listen
use std::net::TcpListener;

// io module==> reading and writing to tcp stream
use std::io::{Read, Write};

// request module
use crate::http::{Request, Response, StatusCode, ParseError};

// convert trait
use std::convert::TryFrom;

// costum trum trait to handle request
pub trait Handler {
    fn handle_request(&mut self, request:&Request)->Response;
    fn handle_bad_request(&mut self, e:&ParseError)->Response{
        println!("Fail to parse the request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }

}



// server module
// Struct 
pub struct Server{
    addr:String,
}

// Struct implementations
impl Server {
    // methods => take an instance "self" => as param
    pub fn run(self, mut handler:impl Handler){
        println!("Server is running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            /*
            let res = listener.accept();
            if res.is_err(){
                continue;
            }
            let (stream, addrclient)= res.unwrap();
            */

            // with match
            match listener.accept() {
                Ok((mut stream, _)) =>{
                    let mut buffer = [0; 1024]; // contains the request content (method - path - if(body)) ==> We have to generate the struct from it
                    match stream.read(&mut buffer) {
                        Ok(_) =>{
                            println!("Received a request : {}", String::from_utf8_lossy(&buffer));
                            let response  = match Request::try_from(&buffer[..]) {
                                Ok(request) =>{
                                    handler.handle_request(&request)
                                },
                                // if can't parse the response
                                Err(e) =>{
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response {}", e);
                            }
                        },
                        Err(e) =>{
                            println!("Failed to read from connexion : {}", e);
                        },
                    }
                },

                Err(e) =>{
                    println!("Failed to established connexion : {}", e);
                },
            }
            
        }
    }

    // associated functions => dont neew an instance / they are associated to the stype
    pub fn new(addr:String) ->Self {

        Self{
            addr:addr,
        }
    }
}

