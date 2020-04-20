use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| App::new());
}
