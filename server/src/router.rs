use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use crate::request::{Request, RequestValue};
use crate::responses::{response_404, response_400};

use crate::endpoints::default;
use crate::endpoints::print;

type RouteHandler = Box<dyn Fn(Request) -> String + Send + Sync>;

pub struct Router {
    routes: HashMap<String, RouteHandler>,
    listener: TcpListener
}

impl Default for Router {
    fn default() -> Self {
        let mut routes = HashMap::new();

        routes.insert("GET / ".to_string(), Box::new(|req: Request| default::get(req)) as RouteHandler);
        routes.insert("GET /print".to_string(), Box::new(|req: Request| print::get(req)) as RouteHandler);
        routes.insert("POST /print".to_string(), Box::new(|req: Request| print::post(req)) as RouteHandler);

        let listener = TcpListener::bind("127.0.0.1:3000").expect("Cannot start the server.");
        println!("Serveur started on http://127.0.0.1:3000");

        Router { routes, listener }
    }
}

impl Router {
    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_stream(stream);
                }
                Err(e) => eprintln!("Connexion error: {}", e),
            }
        }
    }
    pub fn handle_stream(&self, mut stream: TcpStream) {
        let mut buffer = [0; 2048];

        match stream.read(&mut buffer) {
            Ok(_) => {
                let request = str::from_utf8(&buffer).unwrap_or("");
                println!("Request received:\n{}", request);

                // Trying out custom operators
                let mut parsed_request = Request::new();
                if !parsed_request.load_request(request) {
                    stream.write_all(response_400("HTTP first line is not formatted correctly.").as_bytes()).unwrap();
                }
                // if let RequestValue::Method(method) = &parsed_request["method"] {
                //     println!("Method: {}", method);
                // }
                // if let RequestValue::Headers(headers) = &parsed_request["headers"] {
                //     println!("Header: {}", headers["Accept"]);
                // }
                // if let RequestValue::Parameters(params) = &parsed_request["params"] {
                //     println!("Parameter: {:?}", params["test"]);
                // }
                // if let RequestValue::Route(route) = &parsed_request["route"] {
                //     println!("Parameter: {}", route);
                // }

                let response = self.handle_endpoint(parsed_request);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => eprintln!("Error while reading request: {}", e),
        }
    }
    pub fn handle_endpoint(&self, request: Request) -> String {
        for endpoint in self.routes.keys() {
            match (&request["method"], &request["route"]) {
                (RequestValue::Method(method), RequestValue::Route(route)) => {
                    let split_endpoint: Vec<&str> = endpoint.split(' ').collect();
                    if split_endpoint[0].starts_with(method) && route.starts_with(split_endpoint[1]) {
                        return (self.routes[endpoint])(request);
                    }
                }
                _ => {}
            }
        }
        response_404()
    }
}