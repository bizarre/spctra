use crate::core::backend::{Backend, Service};
use crate::core::server::Server;
use crate::minecraft::server::{ MinecraftServer, MinecraftServerSnapshot };
use mcio;

pub struct MinecraftBackend;

impl Backend<MinecraftServer, MinecraftServerSnapshot> for MinecraftBackend {
    fn fetch(server: MinecraftServer) -> Result<MinecraftServerSnapshot, ()> {
        let _response = mcio::ping(server.get_address(), server.get_port(), 315);

        Err(())
    }

    fn get_services() -> Vec<Box<dyn Service>> {
        vec![]
    }
}
