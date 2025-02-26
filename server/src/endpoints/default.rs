use crate::responses::{response_200};
use crate::request::{Request};

pub fn get(_: Request) -> String {
    response_200("Bienvenue sur mon serveur Rust!")
}