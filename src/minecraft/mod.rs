pub mod backend;
pub mod server;

use crate::core::backend::{ Backend, Config };
use crate::core::server::{ Server, ServerSnapshot };
use actix_web::{ web, App, HttpResponse, HttpServer, Responder };

pub fn init(config: String) -> backend::MinecraftBackend {
    backend::MinecraftBackend::new(
        backend::MinecraftConfig::parse(config)
    )
}