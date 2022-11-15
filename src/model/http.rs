use std::io::Cursor;
use serde::{Serialize};


use okapi::{openapi3::Responses, Map};
use rocket::{Request, Response, http::{Header, Status, ContentType}, response::{Responder}};
use rocket_okapi::{response::{OpenApiResponderInner}, gen::OpenApiGenerator, OpenApiError};

pub struct MessageResponder<A> where A: ToString {
    pub content: Option<A>,
    message: Option<String>,
    status: Status,
}

impl <A> MessageResponder<A> where A: ToString {
    #[allow(unused)]
    pub fn create(status: Status, content: A, message: String) -> MessageResponder<A> {
        MessageResponder{ content: Some(content), message: Some(message), status: status }
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

impl <'r, A> Responder<'r, 'static> for MessageResponder<A> where A: ToString {
    fn respond_to(self, _request: &'r Request<'_>) ->  rocket::response::Result<'static> {

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

impl <A> OpenApiResponderInner for MessageResponder<A> where A: ToString {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)\n\
                The request given is wrongly formatted or data asked could not be fulfilled. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [404 Not Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)\n\
                This response is given when you request a resource that does not exists.\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)\n\
                This response is given when something wend wrong on the server. \
                ".to_string(),
                ..Default::default()
            }),
        );

        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

pub struct KeyValueListResponder<A, B> where A: ToString + Serialize, B: ToString + Serialize {
    status: Status,
    data: Vec<(A, B)>,
}

impl <A, B> KeyValueListResponder<A, B> where A: ToString + Serialize, B: ToString + Serialize {
    pub fn create(status: Status, data: Vec<(A, B)>) -> KeyValueListResponder<A, B> {
        KeyValueListResponder { status, data }
    }

    pub fn create_ok(data: Vec<(A, B)>) -> KeyValueListResponder<A, B> {
        KeyValueListResponder::create(Status::Ok, data)
    }
}

impl <'r, A, B> Responder<'r, 'static> for KeyValueListResponder<A, B> where A: ToString + Serialize, B: ToString + Serialize {
    fn respond_to(self, _request: &'r Request<'_>) ->  rocket::response::Result<'static> {

        let mut response = Response::new();
        let body = self.data;
        let json_body_result = serde_json::to_string(&body);
        response.set_header(ContentType::JSON);
        if json_body_result.is_err() {
            response.set_header(Header::new("message", json_body_result.unwrap_err().to_string()));
            response.set_status(Status::InternalServerError);
            return Ok(response);
        }

        let json_body = json_body_result.unwrap();
        response.set_status(self.status);
        response.set_sized_body(json_body.len(), Cursor::new(json_body));

        Ok(response)
    }

}

impl <A, B> OpenApiResponderInner for KeyValueListResponder<A, B> where A: ToString + Serialize, B: ToString + Serialize {

    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)\n\
                The request given is wrongly formatted or data asked could not be fulfilled. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [404 Not Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)\n\
                This response is given when you request a resource that does not exists.\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)\n\
                This response is given when something wend wrong on the server. \
                ".to_string(),
                ..Default::default()
            }),
        );

        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}
