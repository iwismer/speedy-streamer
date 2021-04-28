#![allow(dead_code)]
mod client;
mod client_connector;
mod client_pool;
mod pipe_reader;

pub type Client = client::Client;
pub type ClientConnector = client_connector::ClientConnector;
pub type PipeReader = pipe_reader::PipeReader;
pub type ClientPool = client_pool::ClientPool;
