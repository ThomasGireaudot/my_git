use crate::responses::{response_200, response_400};
use crate::request::{Request, RequestValue, BodyType};

pub fn get(request: Request) -> String {
    response_200("GET /print successfull")
}

pub fn post(request: Request) -> String {
    if let RequestValue::Body(body) = &request["body"] {
        match body {
            BodyType::Json(json) => {
                println!("{:?}", json);
            },
            _ => ()
        }
        response_200("POST /print successfull")
    } else {
        response_400("POST /print failed")
    }
}