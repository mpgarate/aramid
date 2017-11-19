use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::Write;

use request::Request;
use thread_pool::ThreadPool;
use router::{Route, Router, Handler};

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

    pub fn handle(&mut self, path: &str, handler: Handler) {
        self.router.add_route(Route::new(path, handler));
    }

}

fn handle_client(mut stream: TcpStream, router: Router) {
    let mut request = Request::from_tcp_stream(&stream);
    let route = router.get_route(&request);
    let response_string = route.handle(&mut request).as_http_string();

    let _ = stream.write(response_string.as_bytes());
}
