use actix_web::{web, App, HttpResponse, HttpServer};

use crate::cli::args::ServerArgs;
pub async fn run(args: ServerArgs) -> std::io::Result<()> {
    let port = args.port.unwrap_or(8080);
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
