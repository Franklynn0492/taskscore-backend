use std::io::Cursor;

use rocket::{Request, Response, http::{Header, Status}, response::Responder};



pub struct MessageResponder<A> where A: ToString {
    content: Option<A>,
    message: Option<String>,
    status: Status
}

impl <A> MessageResponder<A> where A: ToString {
    #[allow(unused)]
    pub fn create(_status: Status, content: A, message: String) -> MessageResponder<A> {
        MessageResponder{ content: Some(content), message: Some(message), status: Status::Ok }
    }

    pub fn create_with_message(status: Status, message: String) -> MessageResponder<A> {
        MessageResponder{ content: None, message: Some(message), status }
    }

    pub fn create_ok(content: A) -> MessageResponder<A> {
        MessageResponder{ content: Some(content), message: None, status: Status::Ok }
    }

    #[allow(unused)]
    pub fn create_ok_empty() -> MessageResponder<A> {
        MessageResponder{ content: None, message: None, status: Status::Ok }
    }
}

impl <'r, 'o: 'r, A> Responder<'r, 'o> for MessageResponder<A> where A: ToString {
    fn respond_to(self, _request: &'r Request<'_>) ->  rocket::response::Result<'o> {

        let mut response = Response::new();
        response.set_status(self.status);
        if self.message.is_some() {
            response.set_header(Header::new("message", self.message.unwrap()));
        }
        if self.content.is_some() {
            let res = self.content.unwrap();
            let body = res.to_string();
            response.set_sized_body(body.len(), Cursor::new(body));
        }

        Ok(response)
    }
}
