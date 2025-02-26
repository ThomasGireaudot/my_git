use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use crate::request::{Request, RequestValue};
use crate::responses::{response_404, response_400};

use crate::endpoints::default;
use crate::endpoints::test;

type RouteHandler = Box<dyn Fn(Request) -> String + Send + Sync>;

pub struct Route {
    method: String,
    endpoint: String,
    handler: RouteHandler
}

impl Route {
    pub fn new(method: &str, endpoint: &str, handler: RouteHandler) -> Self {
        Route {
            method: method.to_string(),
            endpoint: endpoint.to_string(),
            handler
        }
    }
}

pub struct Router {
    routes: Vec<Route>,
    listener: TcpListener
}

macro_rules! add_route {
    ($routes:expr, $method:literal, $endpoint:literal, $handler:path) => {
        $routes.push(Route::new(
            $method,
            $endpoint,
            Box::new(|req: Request| $handler(req)) as RouteHandler
        ));
    };
}

impl Default for Router {
    fn default() -> Self {
        let mut routes: Vec<Route> = Vec::new();

        add_route!(routes, "GET", "/", default::get);
        add_route!(routes, "GET", "/test", test::get);
        add_route!(routes, "POST", "/test", test::post);

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
    fn handle_stream(&self, mut stream: TcpStream) {
        let mut buffer = [0; 2048];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                // using bytes_read to avoid reading the whole buffer
                let request = &str::from_utf8(&buffer).unwrap_or("")[..bytes_read];
                // println!("Request received:\n{}", request);

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
    fn handle_endpoint(&self, request: Request) -> String {
        match (&request["method"], &request["route"]) {
            (RequestValue::Method(method), RequestValue::Route(route)) => {
                for current_route in &self.routes {
                    if current_route.method.starts_with(method) && route.starts_with(&current_route.endpoint) {
                        return (current_route.handler)(request);
                    }
                }
            }
            _ => {}
        }
        response_404()
    }
}