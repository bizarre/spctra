#![crate_name = "spctra"]

#[cfg(feature = "minecraft")]
pub mod minecraft;

pub mod core;

use crate::core::backend::Backend;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:1337")?
    .run()
    .await
}
