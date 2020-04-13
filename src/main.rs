#![crate_name = "spctra"]

#[cfg(feature = "minecraft")]
pub mod minecraft;

pub mod core;

use crate::core::backend::{ Backend, Config };
use crate::core::server::{ Server, ServerSnapshot };

use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use std::env;
use std::fs;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(feature = "minecraft")]
fn load_minecraft_backend(contents: String) {
    let backend = minecraft::backend::MinecraftBackend::new(
        minecraft::backend::MinecraftConfig::parse(contents)
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let config_loc = args.get(1)
        .expect("Configuration file path not provided.");
    
    let _contents = fs::read_to_string(config_loc)
        .expect("Invalid or missing configuration file.");

    #[cfg(feature = "minecraft")]
    load_minecraft_backend(_contents);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
