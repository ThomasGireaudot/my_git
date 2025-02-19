use std::collections::HashMap;
use std::ops::Index;

pub enum RequestValue {
    Method(String),
    Headers(HashMap<String, String>),
    Parameters(HashMap<String, String>)
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

fn print_type<T: ?Sized>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

impl Request {
    pub fn new(request: &str) -> Self {
        let mut data = HashMap::new();
        data.insert(String::from("headers"), RequestValue::Headers(HashMap::new()));
        data.insert(String::from("params"), RequestValue::Parameters(HashMap::new()));
    
        let mut iterator = request.lines();

        let method = RequestValue::Method(Request::parse_method(iterator.next().unwrap_or("")).to_string());
        data.insert(String::from("method"), method);
    
        let mut line = iterator.next().unwrap_or("");
        while !line.is_empty() && !line.starts_with("body") {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() == 2 {
                if let Some(RequestValue::Headers(headers)) = data.get_mut("headers") {
                    headers.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                    println!("Header : {}: {}", parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
            line = iterator.next().unwrap_or("");
        }
        Request { data }
    }
    pub fn parse_method<'a>(endpoint_requested: &'a str) -> &'a str {
        endpoint_requested.split_once(' ').map(|(first, _)| first).unwrap_or("NONE")
    }
}