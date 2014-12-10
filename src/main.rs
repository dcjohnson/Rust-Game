extern crate little_rust_tcp;
extern crate lib;
use little_rust_tcp::tcpserver::ServerFunction;
use lib::server_data::ServerDataConstructor;
use lib::server_data::DataAnalyzer;
use lib::game_data;
use lib::sdl_data;
use std::string::String;
use std::sync::{Arc, Mutex};
use std::collections::ring_buf::RingBuf;

fn main()
{
    let shared_ring_buf = Arc::new(Mutex::new(RingBuf::new()));
    let server_data = ServerDataConstructor::new(shared_ring_buf.clone());
    let data_analyzer = DataAnalyzer::new(shared_ring_buf.clone());
    let server = ServerFunction::new(String::from_str("8000"), String::from_str("0.0.0.0"));
    init_network(server, server_data, data_analyzer);
}

fn init_network(server: little_rust_tcp::tcpserver::Server,
    server_data: lib::server_data::ServerDataStruct,
    mut data_analyzer: lib::server_data::DataAnalyzerStruct)
{
    spawn(proc(){
        server.start_server(server_data);
    });
    spawn(proc(){
        data_analyzer.interpret_data();
    });
}
