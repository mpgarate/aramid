extern crate htttp;
use htttp::server::HTTTPServer;

const NUM_WORKERS: usize = 15;

fn main() {
    HTTTPServer::new(NUM_WORKERS).listen("127.0.0.1:8080");
}
