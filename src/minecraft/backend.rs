use crate::core::backend::{Backend, Service, Config};
use crate::core::server::Server;
use crate::minecraft::server::{ MinecraftServer, MinecraftServerSnapshot };
use mcio;
use serde::Deserialize;

pub struct MinecraftBackend {
    servers: Vec<MinecraftServer>
}

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

impl Backend for MinecraftBackend {
    type T = MinecraftServer;
    type S = MinecraftServerSnapshot;
    type C = MinecraftConfig;
    
    fn new(config: MinecraftConfig) -> Self {
        MinecraftBackend { 
            servers: config.servers.iter().map(|s| s.to_server()
                .expect(&format!("Invalid server: {}", s.id))).collect::<Vec<_>>()
        }
    }
    
    fn fetch(server: MinecraftServer) -> Result<MinecraftServerSnapshot, ()> {
        let _response = mcio::ping(server.get_address(), server.get_port(), 315);

        Err(())
    }

    fn get_services() -> Vec<Box<dyn Service>> {
        vec![]
    }

    fn get_servers(&self) -> &Vec<MinecraftServer> {
        &self.servers
    }
}

impl Config for MinecraftConfig {
    fn parse<S: Into<String>>(data: S) -> Self {
        toml::from_str(&data.into()).expect("Invalid configuration.")
    }
}

impl MinecraftServerDescriptor {
    fn to_server(&self) -> Option<MinecraftServer> {
        MinecraftServer::new(&self.id, &self.name, &self.address, self.port)
    }
}
