extern crate little_rust_tcp;
use std::string::String;
use little_rust_tcp::tcpserver::ServerFunction;
use little_rust_tcp::data::Data;
use std::io::TcpStream;

fn main()
{
    let x = DataStruct{x: String::from_str("Hello")};
    let server = ServerFunction::new(String::from_str("8000"));
    server.start_server(x);
}

struct DataStruct
{
    x: String
}

impl Data for DataStruct
{
    fn process_request_data(&self, mut request: TcpStream)
    {
        request.write(b"Hello World!!\r\n").unwrap();
    }
}
