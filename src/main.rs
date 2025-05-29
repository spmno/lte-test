use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tracing_appender::{non_blocking, rolling::{self}};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use log::{info, error};

#[get("/")]
async fn hello() -> impl Responder {
    info!("Get Hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/position")]
async fn position(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //start log
    let file_appender = rolling::daily("logs", "lte_test.log");
    //let file_appender = BasicRollingFileAppender::new("./logs", RollingConditionBasic::new().daily(), MAX_FILE_COUNT).unwrap();
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    let debug_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let console_subscriber = fmt::layer().with_writer(std::io::stdout).with_filter(debug_filter);

    tracing_subscriber::registry().with(console_subscriber).with(file_layer).init();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(position)
    })
    .bind(("0.0.0.0", 8111))?
    .run()
    .await
}