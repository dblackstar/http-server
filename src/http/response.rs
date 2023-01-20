use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Write, Result as IoResult};
use std::net::TcpStream;


// status code are finite set of number ==> so we can use enums here
use super::StatusCode;


// response struct hold => status code + body
#[derive(Debug)]
pub struct Response{
    status_code : StatusCode, // enum

    // body can be anything ==> json / image / content of httpfile / or empty
    // we make it a string to support all cases
    // string will be wrap in an option ==> to support when bpdy is empty
    body : Option<String>,
}


// constructor for creating a response
impl Response {
    // create a response
    pub fn new(status_code:StatusCode, body:Option<String>) -> Self{

        Response { 
            // i could use shorthand 
            status_code: status_code, body: body 
        }
    }


    // send the response
    pub fn send(&self, stream:&mut dyn Write)-> IoResult<()>{

        let body = match &self.body {
            Some(b) => b,
            None => "",      
        };

        // the return variable
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_pharse(), body)
    }
}
