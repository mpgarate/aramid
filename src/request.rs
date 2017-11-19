use std::net::TcpStream;
use std::io::Read;

#[derive(Debug)]
pub struct Request {
    raw_request: String,
}

impl Request {
    pub fn from_tcp_stream(stream: &TcpStream) -> Request {
        let mut prev_char: Option<char> = None;

        let raw_request = stream.bytes()
            .map(|r| r.unwrap_or('�' as u8) as char)
            .filter(|c| c != &'\r')
            .take_while(|c| {
                if (prev_char == Some('\n')) && (c == &'\n') {
                    return false;
                }

                prev_char = Some(*c);

                true
            })
        .collect();

        Request {
            raw_request: raw_request,
        }
    }
}
