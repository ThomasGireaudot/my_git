use std::collections::HashMap;
use crate::endpoints::default;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use crate::responses::{response_404};

type RouteHandler = Box<dyn Fn(String) -> String + Send + Sync>;

pub struct Router {
    routes: HashMap<String, RouteHandler>,
    listener: TcpListener
}

impl Default for Router {
    fn default() -> Self {
        let mut routes = HashMap::new();

        routes.insert("GET /".to_string(), Box::new(|req: String| default::get(req)) as RouteHandler);

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
            Ok(size) => {
                let request = str::from_utf8(&buffer).unwrap_or("");
                println!("Request received:\n{}", request);
                let response = self.handle_endpoint(request.to_string());
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => eprintln!("Error while reading request: {}", e),
        }
    }
    pub fn handle_endpoint(&self, request: String) -> String {
        for endpoint in self.routes.keys() {
            if request.starts_with(endpoint) {
                return (self.routes[endpoint])(request);
            }
        }
        response_404()
    }
}