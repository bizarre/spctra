use serde::Serialize;

/// Represents a trackable server
pub trait Server : Serialize {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_icon(&self) -> &str;
    fn get_website(&self) -> Option<&String>;
    fn get_record(&self) -> i32;
}

/// Represents a snapshot of a server
pub trait ServerSnapshot<T : Server> : Serialize {
    fn get_count(&self) -> i32;
    fn get_time(&self) -> u64;
}