extern crate aramid_http;
use aramid_http::server::AramidServer;
use aramid_http::request::Request;
use aramid_http::response::Response;

const NUM_WORKERS: usize = 15;

fn main() {
    let mut server = AramidServer::new(NUM_WORKERS);

    server.handle("/foo", {
        fn f(req: &mut Request) -> Response {
            Response::from((200, String::from("hello foo")))
        }; f
    });

    /*
    server.handle("/bar", {
        fn f(req: &mut Request) -> Response {
            Response::from(aramid::status::NoContent)
        }; f
    });
    */

    server.listen("127.0.0.1:8080");
}
