extern crate htttp;
use htttp::ThreadPool;

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;

fn main() {
	let pool = ThreadPool::new(15);

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	// accept connections and process them serially
	for stream in listener.incoming() {
		pool.execute(|| {
			handle_client(stream.unwrap());
		});
	}
}

fn read_http(stream: &TcpStream) -> String {
	stream.bytes()
		.map(|r| r.unwrap_or('�' as u8) as char)
		.filter(|c| c != &'\r')
		.take_while(|c| c != &'\n')
		.collect()
}

fn handle_client(mut stream: TcpStream) {
	loop {
		let s = read_http(&stream);
		println!("{:?}", s);

		if s.is_empty() {
			break;
		}
	}

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

