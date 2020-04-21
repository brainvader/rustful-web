use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    let mut builder = HttpResponse::Ok();
    let mime_type = ContentType::plaintext().to_string();
    builder
        .content_type(mime_type)
        .body("Welcom to Rustful World!")
}

async fn page_not_found() -> impl Responder {
    let mut builder = HttpResponse::NotFound();
    let response = builder.body("404 Not Found");
    response
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let ip = std::net::IpAddr::V4(localhost);
    let port = 8080;
    let addr = std::net::SocketAddr::new(ip, port);
    // let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 443));

    let app_factory = || {
        App::new()
            .route("/", web::get().to(index))
            .default_service(web::route().to(page_not_found))
    };

    let mut server = HttpServer::new(app_factory);

    server.bind(addr)?.run().await
}
