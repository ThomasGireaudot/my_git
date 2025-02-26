use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use serde_json::Value;

pub enum BodyType {
    Json(Value),
    Text(String)
}

pub enum RequestValue {
    Method(String),
    Route(String),
    Headers(HashMap<String, String>),
    Parameters(HashMap<String, Vec<String>>),
    Body(BodyType)
}

pub struct Request {
    data: HashMap<String, RequestValue>
}

impl Index<&str> for Request {
    type Output = RequestValue;

    fn index(&self, index: &str) -> &Self::Output {
        &self.data[index]
    }
}

// impl IndexMut<&str> for Request {
//     fn index_mut(&mut self, index: &str) -> &mut Self::Output {
//         self.data.get_mut(index).expect("Key not found")
//     }
// }

fn print_type<T: ?Sized>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

impl Request {
    pub fn new() -> Self {
        let data = HashMap::new();
        Request { data }
    }
    fn parse_method(&mut self, method_requested: &str) {
        let method_value = RequestValue::Method(method_requested.to_string());
        self.data.insert(String::from("method"), method_value);
    }
    fn parse_route(&mut self, route: &str) {
        // split endpoint and parameters
        let route_elements: Vec<&str> = route.split('?').collect();
        self.data.insert(String::from("route"), RequestValue::Route(route_elements[0].to_string()));
        if route_elements.len() == 2 {
            self.data.insert(String::from("params"), RequestValue::Parameters(HashMap::new()));
            // split parameters
            let params: Vec<&str> = route_elements[1].split('&').collect();
            for param in params {
                // split param name and values
                let param_parts: Vec<&str> = param.split('=').collect();
                let key = param_parts[0].to_string();
                // split values
                let values_split: Vec<&str> = param_parts[1].split(',').collect();
                if let RequestValue::Parameters(params) = self.data.get_mut("params").unwrap() {
                    for split in values_split {
                        if let Some(values) = params.get_mut(&key) {
                            values.push(split.to_string());
                        } else {
                            params.insert(key.clone(), vec![split.to_string()]);
                        }
                    }
                }
            }
        }
    }
    fn parse_headers(&mut self, iterator: &mut std::str::Lines) {
        self.data.insert(String::from("headers"), RequestValue::Headers(HashMap::new()));
        let mut line = iterator.next().unwrap_or("");
        while !line.is_empty() && !line.starts_with("{") {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() == 2 {
                if let Some(RequestValue::Headers(headers)) = self.data.get_mut("headers") {
                    headers.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                    // println!("Header : {}: {}", parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
            line = iterator.next().unwrap_or("");
        }
    }
    fn parse_body(&mut self, body: &str) {
        if let Some(RequestValue::Headers(headers)) = self.data.get("headers") {
            match headers["Content-Type"].as_str() {
                "application/json" => {
                    let json = serde_json::from_str(body).expect("Failed to parse JSON body.");
                    self.data.insert(String::from("body"), RequestValue::Body(BodyType::Json(json)));
                },
                _ => {
                    self.data.insert(String::from("body"), RequestValue::Body(BodyType::Text(body.to_string())));
                }
            }
        }
    }
    pub fn load_request(&mut self, request: &str) -> bool {
        let mut iterator = request.lines();
        let http_endpoint: Vec<&str> = iterator.next().unwrap_or("").split(' ').collect();
        if http_endpoint.len() != 3 || http_endpoint[2] != "HTTP/1.1" {
            return false;
        }
        self.parse_method(http_endpoint[0]);
        self.parse_route(http_endpoint[1]);
        self.parse_headers(&mut iterator);
        match (self.data.get("method"), self.data.get("headers")) {
            (Some(RequestValue::Method(method)), Some(RequestValue::Headers(headers))) => {
                if method == "POST" {
                    self.parse_body(iterator.collect::<String>().as_str());
                    // self.data.insert(String::from("body"), RequestValue::Body(Body::new(&headers["Content-Type"].as_str())));
                    // if let Some(RequestValue::Body(body)) = self.data.get_mut("body") {
                    //     body.parse(iterator.collect::<String>().as_str());
                    // }
                }
            },
            (_, _) => {}
        }
        return true;
    }
}