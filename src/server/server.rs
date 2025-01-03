use crate::cli::args::ServerArgs;

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;

#[derive(serde::Deserialize)]
struct EncodeRequest {
    plaintext: String,
    reflector: String,
    rotorL: String,
    rotorC: String,
    rotorR: String,

    plugboard: Option<String>,
}

#[derive(serde::Serialize)]
struct EncodeResponse {
    result: String,
}

/// Starts the server with an optional port (defaults to 8080)
pub async fn run(args: ServerArgs) -> std::io::Result<()> {
    let port = args.port.unwrap_or(8080);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .service(encode_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[post("/encode")]
async fn encode_handler(data: web::Json<EncodeRequest>) -> impl Responder {
    let result = &data.plaintext;
    HttpResponse::Ok().json(result)
}
