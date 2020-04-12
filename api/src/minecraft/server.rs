use crate::core::server::{ Server, ServerSnapshot };


pub struct MinecraftServer {
    address: String,
    port: u16,
    icon: String,
    motd: String,
    record: i32,
    website: Option<String>,
}

impl MinecraftServer {
    pub fn new<A: Into<String>>(address: A, port: u16) -> Self {
        MinecraftServer {
            address: address.into(),
            port: port,
            icon: "".to_owned(),
            motd: "".to_owned(),
            record: 0,
            website: None
        }
    }

    pub fn get_motd(&self) -> &str { &self.motd }
    pub fn get_port(&self) -> u16 { self.port }
}

impl Server for MinecraftServer {
    fn get_address(&self) -> &str { &self.address }
    fn get_icon(&self) -> &str { &self.icon }
    fn get_website(&self) -> Option<&String> { self.website.as_ref() }
    fn get_record(&self) -> i32 { self.record }
}

pub struct MinecraftServerSnapshot {
    count: i32,
    time: u64
}

impl ServerSnapshot<MinecraftServer> for MinecraftServerSnapshot {
    fn get_count(&self) -> i32 { self.count }
    fn get_time(&self) -> u64 { self.time }
}