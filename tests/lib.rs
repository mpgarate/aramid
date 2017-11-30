extern crate aramid;

#[cfg(test)]
mod test {
    use aramid::http::{Server, Request, ResponseWriter};
    use aramid::http_status;
    use std::io::{BufWriter, BufReader, Write};
    use std::cell::RefCell;

    #[test]
    fn test_server() {
        let mut server = Server::new();

        fn foo_handler(w: &mut ResponseWriter, r: &mut Request) {
            w.write_status_header(http_status::OK);
            w.write_body(String::from("response body sentinel"));
        };

        server.set_handler("/foo", foo_handler);

        let mut request_body = "request body sentinel";
        let mut writer = BufWriter::new(vec![]);

        server.handle("/foo", &mut BufReader::new(request_body.as_bytes()), &mut writer);

        let expected_string = String::from("response body sentinel");
        let actual_string = String::from_utf8(writer.get_ref().to_owned()).unwrap();

        assert_eq!(expected_string, actual_string);
    }
}
