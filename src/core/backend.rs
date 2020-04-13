use crate::core::server::{ Server, ServerSnapshot };


/// Represents a spctra backend
pub trait Backend<T, S, C> where T : Server, S : ServerSnapshot<T>, C : Config {
    fn new(config: C) -> Self;
    fn fetch(server: T) -> Result<S, ()>;
    fn get_services() -> Vec<Box<dyn Service>>;
}

pub trait Config {
    fn parse<S: Into<String>>(data: S) -> Self;
}

pub trait Service {

}