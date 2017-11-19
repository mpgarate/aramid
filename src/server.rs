use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::Write;

use http_request::HTTPRequest;
use thread_pool::ThreadPool;

pub struct HTTTPServer {
    pool: ThreadPool,
}

impl HTTTPServer {
    pub fn new(num_workers: usize) -> HTTTPServer {
        HTTTPServer {
            pool: ThreadPool::new(num_workers),
        }
    }

    pub fn listen<A: ToSocketAddrs>(self, addr: A) {
        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            println!("executing...");
            self.pool.execute(|| {
                handle_client(stream.unwrap());
            });
            println!("executed");
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let request = HTTPRequest::from_tcp_stream(&stream);

    println!("{:?}", request);

    let response = "HTTP/1.1 200 OK
Date: Mon, 27 Jul 2009 12:28:53 GMT
Server: Apache
Last-Modified: Wed, 18 Nov 2017 00:00:00 GMT
Accept-Ranges: bytes
Content-Length: 51
Vary: Accept-Encoding
Content-Type: text/plain

Hello World! My payload includes a trailing CRLF.\r\n".as_bytes();

    let _ = stream.write(response);
}

