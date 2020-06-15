pub mod float;
pub mod integer;
mod simple8b;
pub mod timestamp;

/// Max number of bytes needed to store a varint-encoded 64-bit integer.
const MAX_VAR_INT_64: usize = 10;
