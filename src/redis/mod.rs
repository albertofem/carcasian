/// This is the protocol module which contains a vague
/// implementation of the Redis communication protocol
pub mod protocol;

/// This is the Redis driver used to connect to the storage
/// from a Redis command
pub mod driver;