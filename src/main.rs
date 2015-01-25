extern crate little_rust_tcp;
extern crate lib;
use little_rust_tcp::tcpserver::ServerFunction;
use little_rust_tcp::tcpserver::Server;
use lib::server_data::ServerDataConstructor;
use lib::server_data::DataAnalyzer;
use lib::server_data::ServerDataStruct;
use lib::server_data::DataAnalyzerStruct;
use lib::game_data;
use lib::rendering_data;
use std::string::String;
use std::sync::{Arc, Mutex};
use std::collections::RingBuf;
use std::thread::Thread;

fn main()
{
    let shared_ring_buf = Arc::new(Mutex::new(RingBuf::new()));
    let server_data = ServerDataConstructor::new(shared_ring_buf.clone());
    let data_analyzer = DataAnalyzer::new(shared_ring_buf.clone());
    let server = ServerFunction::new(String::from_str("8000"), String::from_str("0.0.0.0"));
    init_network(server, server_data, data_analyzer);
}

fn init_network(server: Server, server_data: ServerDataStruct, mut data_analyzer: DataAnalyzerStruct)
{
    Thread::spawn(move || {
        server.start_server(server_data);
    });
    data_analyzer.interpret_data();
}
