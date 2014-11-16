extern crate little_rust_tcp;
use little_rust_tcp::tcpserver;
use std::string::String;
use little_rust_tcp::tcpserver::ServerFunction;

fn main()
{
    let server = tcpserver::ServerFunction::new(String::from_str("test"), String::from_str("Hello"));
    println!("{} {}", server.get_port(), server.get_ip());
    server.format_ip();
}
