use crate::core::server::{ Server, ServerSnapshot };

/// Represents a spctra backend
pub trait Backend<T, S> where T : Server, S : ServerSnapshot<T> {
    fn fetch(server: T) -> Result<S, ()>;
}