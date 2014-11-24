use std::io::TcpStream;
use little_rust_tcp::data::Data;
use std::collections::ring_buf::RingBuf;
use std::sync::{Arc, Mutex};
use std::string::String;

pub struct DataStruct
{
    unprocessed_data: Arc<Mutex<RingBuf<String>>>,
    request_buffer: RingBuf<String>
}

pub trait DataConstructor
{
    fn new() -> DataStruct;
}

trait PrivateUtilityTrait
{

}

impl DataConstructor for DataStruct
{
    fn new() -> DataStruct
    {
        let unprocessed = Arc::new(Mutex::new(RingBuf::new()));
        let buffer = RingBuf::new();
        return DataStruct{unprocessed_data: unprocessed, request_buffer: buffer};
    }
}

impl Data for DataStruct
{
    fn process_request_data(&mut self, mut request: TcpStream)
    {
        let message = request.read_to_string();
        let someobject = message.ok();
        if (someobject != None)
        {
            let string = someobject.unwrap();
            if (string.len() > 0)
            {
                self.request_buffer.push_back(string);
                println!("{}", self.request_buffer.pop_front().unwrap());
            }
        }
        request.write(b"Hello World\r\n").unwrap();
    }
}
