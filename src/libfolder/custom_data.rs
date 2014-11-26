use std::io::TcpStream;
use little_rust_tcp::data::Data;
use std::collections::ring_buf::RingBuf;
use std::sync::{Arc, Mutex};
use std::string::String;

pub struct DataAnalyzerStruct
{
    unprocessed_data: RingBuf<String>,
    server_data: Arc<Mutex<RingBuf<String>>>
}

pub trait DataAnalyzer
{
    fn new(new_server_data: Arc<Mutex<RingBuf<String>>>) -> DataAnalyzerStruct;
    fn interpret_data(&mut self);
    fn push_request_to_unprocessed(&mut self);
}

impl DataAnalyzer for DataAnalyzerStruct
{
    fn new(new_server_data: Arc<Mutex<RingBuf<String>>>) -> DataAnalyzerStruct
    {
        let ringbuf = RingBuf::new();
        return DataAnalyzerStruct{unprocessed_data: ringbuf, server_data: new_server_data};
    }

    fn interpret_data(&mut self)
    {
        if self.unprocessed_data.len() == 0
        {
            self.push_request_to_unprocessed();
        }
        let request_string = self.unprocessed_data.pop_front();
        if request_string != None
        {
            println!("{}", request_string.unwrap());
        }
    }

    fn push_request_to_unprocessed(&mut self)
    {
        let mut locked_server_data = self.server_data.lock();
        while locked_server_data.len() > 0
        {
            let request_string = locked_server_data.pop_front();
            self.unprocessed_data.push_back(request_string.unwrap());
        }
    }
}

pub struct ServerDataStruct
{
    request_buffer: Arc<Mutex<RingBuf<String>>>
}

pub trait ServerDataConstructor
{
    fn new(request_buffer_new: Arc<Mutex<RingBuf<String>>>) -> ServerDataStruct;
}

impl ServerDataConstructor for ServerDataStruct
{
    fn new(request_buffer_new: Arc<Mutex<RingBuf<String>>>) -> ServerDataStruct
    {
        return ServerDataStruct{request_buffer: request_buffer_new};
    }
}

impl Data for ServerDataStruct
{
    fn process_request_data(&mut self, mut request: TcpStream)
    {
        let message = request.read_to_string();
        let someobject = message.ok();
        if someobject != None
        {
            let string = someobject.unwrap();
            if string.len() > 0
            {
                let mut locked_request_buffer = self.request_buffer.lock();
                locked_request_buffer.push_back(string);
            }
        }
    }
}
