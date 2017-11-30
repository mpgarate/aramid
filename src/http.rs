use http_status::HTTPStatus;
use std::io::{BufWriter, BufReader, Write};
use std::io::prelude::BufRead;

pub struct Server {}

type Handler = fn(w: &mut ResponseWriter, r: &mut Request);

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn set_handler(&mut self, _: &str, _: Handler) { }

    pub fn handle(
        &mut self,
        path: &str,
        reader: &mut BufReader<&[u8]>,
        writer: &mut BufWriter<Vec<u8>>) {

        // TODO: don't just copycat the request body
        // 1 - find the appropriate handler for the path
        // 2 - run the handler

        for line in reader.lines() {
            writer.write(line.unwrap().as_bytes());
        }

        writer.flush();
    }
}
pub struct Request {}
pub struct ResponseWriter {}

impl ResponseWriter {
    pub fn write_status_header(&mut self, _: HTTPStatus) { }
    pub fn write_body(&mut self, _: String) {
        // TODO: accept a reader
    }
}
