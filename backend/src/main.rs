use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcom to Rustful World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let ip = std::net::IpAddr::V4(localhost);
    let port = 8080;
    let addr = std::net::SocketAddr::new(ip, port);
    // let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 443));

    let app_factory = || App::new().route("/", web::get().to(index));

    let mut server = HttpServer::new(app_factory);

    server.bind(addr)?.run().await
}
