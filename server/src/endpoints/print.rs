use crate::responses::{response_200};
use crate::request::{Request};

pub fn get(request: Request) -> String {
    response_200("GET /print successfull")
}

pub fn post(request: Request) -> String {
    response_200("POST /print successfull")
}