extern crate little_rust_tcp;
use std::string::String;
use little_rust_tcp::tcpserver::ServerFunction;

fn main()
{
    let server = ServerFunction::new(String::from_str("test"), String::from_str("Hello"));
    server.start_server();
}
