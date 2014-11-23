extern crate little_rust_tcp;
extern crate lib;
use std::string::String;
use little_rust_tcp::tcpserver::ServerFunction;
use std::sync::{Arc, Mutex};
use lib::custom_data::CustomDataFunctions;


fn main()
{

    let mut data = CustomDataFunctions::new();
    let mut x = String::from_str("Hell");
    data.push(x);
    let server = ServerFunction::new(String::from_str("8010"));
    server.start_server(data);
}
