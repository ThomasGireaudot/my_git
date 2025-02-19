mod responses;
mod router;
mod endpoints;
mod request;

fn main() {
    let router = router::Router::default();
    router.listen();
}
