extern crate htttp;
use htttp::server::HTTTPServer;
use htttp::server::Route;
use htttp::http_request::HTTPRequest;

const NUM_WORKERS: usize = 15;

fn hello_world(req: &mut HTTPRequest) -> String {
    String::from("hello, world")
}

fn main() {
    let mut server = HTTTPServer::new(NUM_WORKERS);

    server.handle(Route {
        path: String::from("/foo"),
        handler: hello_world,
    });

    server.listen("127.0.0.1:8080");
}
