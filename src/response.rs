use chrono::prelude::{Utc};

pub struct Response {
    code: usize,
    body: String,
}

const TERMINATOR: &'static str = "\r\n";

impl Response {
    pub fn ok(body: &str) -> Response {
        Response {
            code: 200,
            body: String::from(body),
        }
    }

    pub fn as_http_string(&self) -> String {
        let d = vec![
            self.status_line(),
            self.date("Date"),
            self.server(),
            self.date("Last-Modified"),
            self.content_length(),
            self.accept_ranges(),
            self.content_type(),
            self.terminator(),
            self.body.clone(),
            self.terminator(),
        ];

        d.join(TERMINATOR)
    }

    fn date(&self, label: &str) -> String {
        format!("{}: {}", label, Utc::now().format("%a, %e %b %Y %T GMT"))
    }

    fn status_line(&self) -> String {
        format!("HTTP/1.1 {} OK", self.code)
    }

    fn server(&self) -> String {
        String::from("Server: Aramid")
    }

    fn accept_ranges(&self) -> String {
        String::from("Accept-Ranges: none")
    }

    fn content_length(&self) -> String {
        let body_len = self.body.as_bytes().len();
        let term_len = TERMINATOR.len();

        format!("Content-Length: {}", body_len + (3 * term_len))
    }

    fn content_type(&self) -> String {
        format!("Content-Type: {}", "text/plain")
    }

    fn terminator(&self) -> String {
        String::from(TERMINATOR)
    }
}
