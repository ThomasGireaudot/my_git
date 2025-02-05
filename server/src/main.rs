mod responses;
mod router;
mod endpoints;

use std::fs::File;
use std::io::{Read, Write};
use std::str;
use regex::Regex;
use responses::{response_200, response_400, response_404};

fn main() {
    let mut router = router::Router::default();
    router.listen();
}
