extern crate little_rust_tcp;
extern crate lib;
use std::string::String;
use little_rust_tcp::tcpserver::ServerFunction;
use lib::custom_data::ServerDataConstructor;
use lib::custom_data::DataAnalyzer;
use std::sync::{Arc, Mutex};
use std::collections::ring_buf::RingBuf;

fn main()
{
    let shared_ring_buf = Arc::new(Mutex::new(RingBuf::new()));
    let server_data = ServerDataConstructor::new(shared_ring_buf.clone());
    let mut data_analyzer = DataAnalyzer::new(shared_ring_buf.clone());
    let server = ServerFunction::new(String::from_str("8000"));
    spawn(proc(){
        server.start_server(server_data);
    });
    loop
    {
        data_analyzer.interpret_data();
    }
}
