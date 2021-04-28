use futures::{future::select_all, future::Future, future::FutureExt, pin_mut};
use std::pin::Pin;
use tokio::sync::mpsc;

mod models;
mod workers;
use models::Message;
use tokio::signal;
use workers::{ClientConnector, ClientPool, PipeReader};

pub async fn signal_handler() {
    signal::ctrl_c().await.unwrap();
}
#[tokio::main]
async fn main() {
    // Bus to send messages to client pool
    let (bus_tx, rx) = mpsc::channel::<Message>(1000);

    // Pool of clients to send messages to
    let client_pool = ClientPool::new(rx);
    // The thread that connects new clients and adds them to the pool
    let connector = ClientConnector::new(10000, bus_tx.clone()).await;

    // The pipe reader, which gets reads from the named pipe and sends it to the client pool
    let mut reader = PipeReader::new(bus_tx.clone());

    let fut_reader = reader.begin().fuse();
    let fut_clients = client_pool.begin().fuse();
    let fut_conn = connector.begin().fuse();
    let fut_sig = signal_handler().fuse();

    pin_mut!(fut_reader, fut_clients, fut_conn, fut_sig);
    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
        vec![fut_reader, fut_clients, fut_conn, fut_sig];
    select_all(futures).await;
    // If any of them finish, end the program as something went wrong
    bus_tx.clone().send(Message::SHUTDOWN).await.unwrap();
}
