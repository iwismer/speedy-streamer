use super::Client;
use crate::models::Message;
use futures::future::join_all;
use tokio::sync::mpsc::Receiver;

/// Contains a vec of all the clients and forwards reads to them
pub struct ClientPool {
    clients: Vec<Client>,
    bus: Receiver<Message>,
}

impl ClientPool {
    pub fn new(bus: Receiver<Message>) -> Self {
        ClientPool {
            clients: Vec::new(),
            bus,
        }
    }

    /// Begin listening for new clients and reads.
    ///
    /// This function should never return.
    pub async fn begin(mut self) {
        loop {
            match self.bus.recv().await.unwrap() {
                Message::CHIP_READ(r) => {
                    let mut futures = Vec::new();
                    for client in self.clients.iter_mut() {
                        futures.push(client.send_read(r.clone()));
                    }
                    let results = join_all(futures).await;
                    // If a client returned an error, remove it from future
                    // transmissions.
                    for r in results.iter() {
                        if r.is_err() {
                            let pos = self
                                .clients
                                .iter()
                                .position(|c| c.get_addr() == r.err().unwrap());
                            if pos.is_some() {
                                self.clients.remove(pos.unwrap());
                            }
                        }
                    }
                }
                Message::SHUTDOWN => {
                    for client in self.clients.iter_mut() {
                        client.exit().await;
                    }
                    return;
                }
                Message::CLIENT(c) => {
                    self.clients.push(c);
                }
            }
        }
    }
}
