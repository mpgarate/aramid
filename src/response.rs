pub struct Response {
    code: usize,
    body: String,
}

impl Response {
    pub fn ok(body: &str) -> Response {
        Response {
            code: 200,
            body: String::from(body),
        }
    }

    pub fn as_http_string(&self) -> String {
        let d = vec![
            String::from(format!("HTTP/1.1 {} OK", self.code.clone())),
            self.body.clone(),
            String::from("\n"),
        ];

        d.join("\r\n")
    }

    /*
    fn status_line(&self) -> String {
        format!("HTTP/1.1 {} OK", self.code)
    }

    fn terminator(&self) -> String {
        String::from("\n")
    }
    */
}

/*
 * HTTP/1.1 200 OK
 Date: Mon, 27 Jul 2009 12:28:53 GMT
 Server: Apache
 Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
 ETag: "34aa387-d-1568eb00"
 Accept-Ranges: bytes
 Content-Length: 51
 Vary: Accept-Encoding
 Content-Type: text/plain

 Hello World! My payload includes a trailing CRLF.
 */
