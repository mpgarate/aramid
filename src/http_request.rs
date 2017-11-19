use std::net::TcpStream;
use std::io::Read;

#[derive(Debug)]
pub struct HTTPRequest {
    raw_request: String,
}

impl HTTPRequest {
    pub fn from_tcp_stream(stream: &TcpStream) -> HTTPRequest {
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

        HTTPRequest {
            raw_request: raw_request,
        }
    }
}
