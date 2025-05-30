use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tracing_appender::{non_blocking, rolling::{self}};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use log::{info, error};
use actix_files as fs;
use sea_orm::{Database, DatabaseConnection, EntityTrait};


#[post("/position")]
async fn position(req_body: String) -> impl Responder {
    info!("{}", req_body);
    HttpResponse::Ok().body("ok")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //start log
    let file_appender = rolling::daily("logs", "lte_test.log");

    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    let debug_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let console_subscriber = fmt::layer().with_writer(std::io::stdout).with_filter(debug_filter);

    tracing_subscriber::registry().with(console_subscriber).with(file_layer).init();
    HttpServer::new(|| {
        App::new()
            .service(position)
           .service(fs::Files::new("/", "./src/static").show_files_listing().index_file("index.html"))
    })
    .bind(("0.0.0.0", 8111))?
    .run()
    .await
}