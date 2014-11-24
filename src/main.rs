extern crate little_rust_tcp;
extern crate lib;
use std::string::String;
use std::sync::{Arc, Mutex};
use little_rust_tcp::tcpserver::ServerFunction;
use lib::custom_data::DataConstructor;
use lib::custom_data::DataStruct;
use std::io::timer;
use std::time::Duration;

fn main()
{
    let mut data = DataConstructor::new();
    let mut server = ServerFunction::new(String::from_str("8000"));
    server.start_server(data);
    // let arc = Arc::new(Mutex::new(CustomDataFunctions::new()));
    // let mut t = CustomDataFunctions::new();
    // t.process_request_data();
}

// fn start_server_proc(data_clone: Arc<Mutex<DataStruct>>)
// {
//     spawn(proc(){
//         let mut locked_data = data_clone.lock();
//         let server = ServerFunction::new(String::from_str("8000"));
//         server.start_server(locked_data);
//     });
// }
