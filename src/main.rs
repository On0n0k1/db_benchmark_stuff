mod dao;
mod error;
mod globals;
mod server;

use actix_web::{get, middleware::Logger, web, HttpResponse, HttpServer, Responder};
use dao::Dao;
use error::Error;
use log::info;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Request {
    num: u32,
}

#[get("/hello")]
pub async fn hello_world() -> Result<impl Responder, Error> {
    info!("Running Hello World");
    Ok(HttpResponse::Ok().body("Hello World"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "Debug");
    env_logger::init();

    let dao: Dao = Dao::new().unwrap();
    let dao: Arc<Dao> = Arc::new(dao);

    info!("Starting server in localhost:9090");
    HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(dao.clone()))
            .service(hello_world)
            .service(server::read_paciente::read_paciente)
            .service(server::read_many_pacients::read_many_pacients)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
