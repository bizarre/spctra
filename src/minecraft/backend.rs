use crate::core::backend::{Backend, Service, Config};
use crate::core::server::Server;
use crate::minecraft::server::{ MinecraftServer, MinecraftServerSnapshot };
use mcio;
use serde::Deserialize;

pub struct MinecraftBackend;

#[derive(Deserialize, Debug)]
struct MinecraftServerDescriptor {
    id: String,
    name: String,
    address: String,
    port: u16
}

#[derive(Deserialize, Debug)]
pub struct MinecraftConfig {
    servers: Vec<MinecraftServerDescriptor>
}

impl Backend<MinecraftServer, MinecraftServerSnapshot, MinecraftConfig> for MinecraftBackend {
    fn new(config: MinecraftConfig) -> Self {
        MinecraftBackend { }
    }
    
    fn fetch(server: MinecraftServer) -> Result<MinecraftServerSnapshot, ()> {
        let _response = mcio::ping(server.get_address(), server.get_port(), 315);

        Err(())
    }

    fn get_services() -> Vec<Box<dyn Service>> {
        vec![]
    }
}

impl Config for MinecraftConfig {
    fn parse<S: Into<String>>(data: S) -> Self {
        toml::from_str(&data.into()).expect("Invalid configuration.")
    }
}