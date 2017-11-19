use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::Write;

use http_request::HTTPRequest;
use thread_pool::ThreadPool;

pub struct Route {
    pub path: String,
    pub handler: fn(&mut HTTPRequest) -> String,
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route { path: self.path.clone(), handler: self.handler }
    }
}

pub struct HTTTPServer {
    pool: ThreadPool,
    routes: Vec<Box<Route>>,
}

impl HTTTPServer {
    pub fn new(num_workers: usize) -> HTTTPServer {
        HTTTPServer {
            pool: ThreadPool::new(num_workers),
            routes: Vec::new(),
        }
    }

    pub fn listen<A: ToSocketAddrs>(&self, addr: A) {
        let listener = TcpListener::bind(addr).unwrap();

        let route = self.routes.first().unwrap();

        for stream in listener.incoming() {
            let route2 = (**route).clone();

            println!("executing...");
            self.pool.execute(|| {
                handle_client(stream.unwrap(), route2);
            });
            println!("executed");
        }
    }

    pub fn handle(&mut self, route: Route) {
        self.routes.push(Box::new(route));
    }

}

fn handle_client(mut stream: TcpStream, route: Route) {
    let mut request = HTTPRequest::from_tcp_stream(&stream);

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

    let _ = stream.write((route.handler)(&mut request).as_bytes());
}
