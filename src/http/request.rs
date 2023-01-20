

// request  module 
use super::method::{Method, MethodError};

// convert trait
use std::convert::TryFrom;

// Utf8Error trait
use std::str::Utf8Error;

// str module
use std::str;

// Error trait
use std::error::Error;

// Result - formatter - Display => trait
use std::fmt::{Result as FmtResult, Formatter, Display, Debug};

// query string for => /a&=1&b=7&c&d=&e===&d=7&d=abc
use super::{QueryString};


#[derive(Debug)]
pub struct Request<'buff>{
    path : & 'buff str,
    query_string : Option<QueryString<'buff>>,
    method : Method,
}


impl<'buff> Request<'buff> {
    pub fn path(&self) ->&str{
        &self.path
    }

    pub fn method(&self) ->&Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }

}




impl<'buff> TryFrom<& 'buff[u8]> for Request<'buff>{
    // &[u8] => is the slice we want to convert to => Request (the struct)
    type Error =  ParseError; // costum error message .... see bellow type for the return 

    /*
        Eg of http request : 
            GET /search?name=abc&sort=1 HTTP/1.1
    */


// parsing the request
    fn try_from(buf: &'buff [u8]) -> Result<Request<'buff>, Self::Error> {

// convert the slice into request string
        let request = str::from_utf8(buf)?;

// get the first line words => from the request (method)
        // get the METHOD and give the reste of the request => to the next call
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;

        // get the PATH and give the reste of the request => to the next call
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        // get the PROTOCOL and IGNORE the reste of the request 
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;

// check if the protocol is 1.1
        if protocol != "HTTP/1.1" {
            return  Err(ParseError::InvalidProtocol);
        }


// converte method (&str) => Method (Enum)
        let method:Method = method.parse()?; // parse() use to convert 

// separate the path => from the query string
        let mut query_string = None;

        // discover the first (?)

        /*  Method 1
        match path.find('?'){ // return(an option => maybe no match pattern) the byte index of a character of string slice
            Some(i) => {
                query_string = Some(&path[i+1..]); // +1 => we don't want '?'=> to be part of the query_string
                path = &path[0..i];
            },
            None => {},

        }

        Method 2
        let q = path.find('?');
        if q.is_some(){
            let i = q.unwrap();
            query_string = Some(&path[i+1..]); // +1 => we don't want '?'=> to be part of the query_string
            path = &path[0..i];
        }else{
            unimplemented!();
        }

        */
        // Method 3
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..])); // +1 => we don't want '?'=> to be part of the query_string + convert into string Option (check to request struct)
            path = &path[0..i]; // the first part is a path
        }


        // The return value => actually the requst Struct we succeded the creat from the buf 
        Ok(Self {
            // convert slice -> String
            path: path,
            query_string: query_string,
            method,
        })

    }
}



fn get_next_word(request:&str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r' { // => search space (between words) or (\r at the end of the string)
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
    
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}


impl ParseError {
    fn message(&self)-> &str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod  => "Invalid Method", 
        }
    }
}
impl From<MethodError> for ParseError {
    fn from(_:MethodError) -> Self{
        Self::InvalidMethod
    }

}
impl From<Utf8Error> for ParseError {
    fn from(_:Utf8Error) -> Self{
        Self::InvalidEncoding
    }

}

/*
Types that want to implement Error trait => must implement :
    - Display trait 
    - Debug trait

*/
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }   
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }   
}


// finally implementing the error trait
impl Error for ParseError {
    
}