use std::io::TcpStream;
use little_rust_tcp::data::Data;
use std::collections::ring_buf::RingBuf;

pub struct DataStruct
{
    unprocessed_data: RingBuf<String>
}

pub trait CustomDataFunctions
{
    fn new() -> DataStruct;
    fn pop(&mut self) -> Option<String>;
    fn push(&mut self, new_data: String);
}

impl CustomDataFunctions for DataStruct
{
    fn new() -> DataStruct
    {
        return DataStruct{unprocessed_data: RingBuf::new()};
    }

    fn pop(&mut self) -> Option<String>
    {
        return self.unprocessed_data.pop_front();
    }

    fn push(&mut self, new_data: String)
    {
        self.unprocessed_data.push_back(new_data);
    }
}

impl Data for DataStruct
{
    fn process_request_data(&self, mut request: TcpStream)
    {
        request.write(b"test\r\n").unwrap();
    }
}
