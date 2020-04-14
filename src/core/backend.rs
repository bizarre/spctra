use crate::core::server::{ Server, ServerSnapshot };


/// Represents a spctra backend
pub trait Backend {
    type T: Server;
    type S: ServerSnapshot<Self::T>;
    type C: Config;

    fn new(config: Self::C) -> Self;
    fn fetch(server: Self::T) -> Result<Self::S, ()>;
    fn get_servers(&self) -> &Vec<Self::T>;
    fn get_services() -> Vec<Box<dyn Service>>;
}

pub trait Config {
    fn parse<S: Into<String>>(data: S) -> Self;
}

pub trait Service {

}