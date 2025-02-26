use crate::responses::{response_200};

pub fn get(request: String) -> String {
    response_200(&request.as_str())
}

pub fn post(request: String) -> String {
    response_200(&request.as_str())
}