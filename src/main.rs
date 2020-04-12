#![crate_name = "spctra"]

#[cfg(feature = "minecraft")]
pub mod minecraft;

pub mod core;

use crate::core::backend::Backend;

fn main() {
    if cfg!(feature = "minecraft") {
        let server = minecraft::server::MinecraftServer::new("localhost", 25577);
        minecraft::backend::MinecraftBackend::fetch(server);
    }
}
