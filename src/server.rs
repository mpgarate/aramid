use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::Write;

use http_request::HTTPRequest;
use thread_pool::ThreadPool;
use router::{Route, Router};

pub struct HTTTPServer {
    pool: ThreadPool,
    router: Router,
}

impl HTTTPServer {
    pub fn new(num_workers: usize) -> HTTTPServer {
        HTTTPServer {
            pool: ThreadPool::new(num_workers),
            router: Router::new(),
        }
    }

    pub fn listen<A: ToSocketAddrs>(&self, addr: A) {
        let listener = TcpListener::bind(addr).unwrap();


        for stream in listener.incoming() {

            let router = self.router.clone();

            println!("executing...");
            self.pool.execute(|| {
                handle_client(stream.unwrap(), router);
            });
            println!("executed");
        }
    }

    pub fn handle(&mut self, route: Route) {
        self.router.add_route(route);
    }

}

fn handle_client(mut stream: TcpStream, router: Router) {
    let mut request = HTTPRequest::from_tcp_stream(&stream);
    let route = router.get_route(&request);
    let _ = stream.write(route.handle(&mut request).as_bytes());
}
