use crate::responses::{response_200};

pub fn get(_: String) -> String {
    response_200("Bienvenue sur mon serveur Rust!")
}