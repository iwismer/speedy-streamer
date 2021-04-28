use crate::workers::Client;

/// A message that gets passed along the bus between workers
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Message {
    // Shutdown all the streams
    SHUTDOWN,
    // Pass a chip read along to the clients
    CHIP_READ(String),
    // A new client that just connected
    CLIENT(Client),
}
