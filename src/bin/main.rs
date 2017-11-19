extern crate htttp;
use htttp::server::HTTTPServer;
use htttp::request::Request;
use htttp::response::Response;

const NUM_WORKERS: usize = 15;

fn hello_world(_: &mut Request) -> Response {
    Response::ok("hello, world")
}

fn main() {
    let mut server = HTTTPServer::new(NUM_WORKERS);

    server.handle(
        "/foo",
        hello_world,
    );

    server.listen("127.0.0.1:8080");
}
