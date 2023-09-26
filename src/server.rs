use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        eprintln!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        } 
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        'serve: loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Received a request:");

                    let mut full_content: Vec<u8> = Vec::new();
                    let mut buffer = [0; 1024];

                    'read_chunks: loop {
                        match stream.read(&mut buffer) {
                            Ok(count) => {
                                if count == 0 {
                                    // End of stream reached
                                    break;
                                }

                                full_content.extend_from_slice(&buffer[..count]);

                                if count < 1024 {
                                    break;
                                }
                            }
                            Err(err) => {
                                // Error occurred during read
                                eprintln!("Error: {}", err);
                                continue;
                            }
                        }
                    }

                    println!("{:?}", String::from_utf8_lossy(&full_content));

                    let response = match Request::try_from(full_content.as_slice()) {
                        Ok(request) => {
                            handler.handle_request(&request)
                        },
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            handler.handle_bad_request(&err)
                        }
                    };

                    if let Err(e) = response.send(&mut stream) {
                        eprintln!("Failed to send response.");
                    }
                },
                Err(err) => println!("Failed to establish connection: {}", err),
            }
        }
    }
}