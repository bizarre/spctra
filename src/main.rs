#![crate_name = "spctra"]

#[cfg(feature = "minecraft")]
pub mod minecraft;

pub mod core;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use std::env;
use std::fs;
use crate::core::backend::Backend;
use std::sync::Mutex;

async fn index<T : Backend>(data: web::Data<State<T>>) -> impl Responder {
    HttpResponse::Ok().body(format!("Servers: {}", data.backend.lock().unwrap().get_servers().len()))
}

struct State<B: Backend> {
    pub backend: Mutex<B>
}

#[cfg(feature = "minecraft")]
async fn start_minecraft(config: String) -> std::io::Result<()> {
    let state = web::Data::new(State::<minecraft::backend::MinecraftBackend> { 
        backend: Mutex::new(minecraft::init(config))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index::<minecraft::backend::MinecraftBackend>))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let config_loc = args.get(1)
        .expect("Configuration file path not provided.");
    
    let _contents = fs::read_to_string(config_loc)
        .expect("Invalid or missing configuration file.");

    if cfg!(feature = "minecraft") {
        #[cfg(feature = "minecraft")]
        start_minecraft(_contents).await;
    };

    Ok(())
}
