use crate::core::server::{ Server, ServerSnapshot };
use serde::Serialize;
use std::clone::Clone;

#[derive(Debug, Serialize, Clone)]
pub struct MinecraftServer {
    id: String,
    name: String,
    address: String,
    port: u16,
    icon: String,
    motd: String,
    record: i32,
    website: Option<String>,
}

impl MinecraftServer {
    pub fn new<A: Into<String>>(id: A, name: A, address: A, port: u16) -> Option<Self> {
        let address = address.into().to_owned();

        let data = mcio::ping(address.to_owned(), port, 315).unwrap();

        Some(MinecraftServer {
            id: id.into(),
            name: name.into(),
            address: address,
            port: port,
            icon: data.favicon,
            motd: "".to_owned(),
            record: 0,
            website: None
        })
    }

    pub fn get_motd(&self) -> &str { &self.motd }
    pub fn get_port(&self) -> u16 { self.port }
}

impl Server for MinecraftServer {
    fn get_id(&self) -> &str { &self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_address(&self) -> &str { &self.address }
    fn get_icon(&self) -> &str { &self.icon }
    fn get_website(&self) -> Option<&String> { self.website.as_ref() }
    fn get_record(&self) -> i32 { self.record }
}

#[derive(Debug, Serialize)]
pub struct MinecraftServerSnapshot {
    count: i32,
    time: u64
}

impl MinecraftServerSnapshot {
    pub fn new(count: i32) -> Self {
        MinecraftServerSnapshot {
            count: count,
            time: 0
        }
    }
}

impl ServerSnapshot<MinecraftServer> for MinecraftServerSnapshot {
    fn get_count(&self) -> i32 { self.count }
    fn get_time(&self) -> u64 { self.time }
}