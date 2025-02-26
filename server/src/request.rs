use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub enum BodyField {
    List(Vec<BodyField>),
    Text(String)
}

pub enum RequestValue {
    Method(String),
    Headers(HashMap<String, String>),
    Parameters(HashMap<String, Vec<String>>),
    Body(HashMap<String, BodyField>)
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

impl IndexMut<&str> for Request {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.data.get_mut(index).expect("Key not found")
    }
}

fn print_type<T: ?Sized>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

impl Request {
    pub fn new() -> Self {
        let data = HashMap::new();
        Request { data }
    }
    fn parse_method(&mut self, endpoint_requested: &str) {
        let method_str = endpoint_requested.split_once(' ').map(|(first, _)| first).unwrap_or("NONE");
        let method = RequestValue::Method(method_str.to_string());
        self.data.insert(String::from("method"), method);
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
    pub fn load_request(&mut self, request: &str) {
        let mut iterator = request.lines();
        self.parse_method(iterator.next().unwrap_or(""));
        self.parse_headers(&mut iterator);
        if let RequestValue::Method(method) = &self.data["method"] {
            match method.as_str() {
                "GET" => {
                    self.data.insert(String::from("params"), RequestValue::Parameters(HashMap::new()));
                }
                "POST" => {
                    self.data.insert(String::from("body"), RequestValue::Body(HashMap::new()));
                }
                _ => {}
            }
        }
    }
}