use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream};
use std::io::{BufReader, Result, Error, ErrorKind};
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
use std::convert::TryInto;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:4000";

pub fn now() -> u128 {
	let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Request {
    request_num: u64,
    timestamp_send: u128,
    timestamp_receive: u128,
}

impl Request {
    pub fn new_client_request(request_num: u64) -> Self {
        Self{request_num, timestamp_send: now(), timestamp_receive: 0 as u128}
    }

    pub fn new_server_request(request_num: u64, timestamp_receive: u128) -> Self{
        Self{request_num, timestamp_send: now(), timestamp_receive}
    }

    pub fn read_request_num(self) -> u64 {
        return self.request_num;
    }

    pub fn read_timestamp_recieve(self) -> u128 {
        return self.timestamp_receive;
    }

    pub fn read_timestamp_send(self) -> u128 {
        return self.timestamp_send;
    }
}


// theoretically turns vector into equivalent array...but you know, it can mess things up
fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

// Abstracted Protocol that wraps a TcpStream and manages
/// sending & receiving of messages
pub struct Protocol {
    reader: BufReader<TcpStream>,
    stream: TcpStream,
}
impl Protocol {
    /// Wrap a TcpStream with Protocol
    pub fn with_stream(stream: TcpStream) -> Result<Self> {
        Ok(Self {
            reader: BufReader::new(stream.try_clone()?),
            stream,
        })
    }



    /// Establish a connection, wrap stream in BufReader/Writer
    pub fn connect(dest: SocketAddr) -> Result<Self> {
        let stream = TcpStream::connect(dest)?;
        eprintln!("Connecting to {}", dest);
        Self::with_stream(stream)
    }

    /// Serialize a message to the server and write it to the TcpStream
    pub fn send_message(&mut self, message: Request) -> Result<()> {
        let mut serial_message: Vec<u8> = bincode::serialize(&message).unwrap();
        self.stream.write_u16::<NetworkEndian>(serial_message.len() as u16);
        self.stream.write_all(&mut serial_message);
        self.stream.flush();
        Ok(())
    }

    /// Read a message from the inner TcpStream
    ///
    // Odd # = response from server so two timestamps, Even # = request from client so one timestamp
    pub fn read_message(&mut self) -> Result<Request> {
        let length = self.reader.read_u16::<NetworkEndian>()?; // length of the serialization
        let mut request_buf = vec![0u8; 8];
        //Ok(Response{message: response_str})
        self.reader.read_exact(&mut request_buf); // bytes for request number
        let mut time_buf = vec![0u8; 16];
        self.reader.read_exact(&mut time_buf);

        let mut request_num = u64::from_le_bytes(demo(request_buf));
        let mut time2_buf = vec![0u8; 16];
        let timestamp2: u128;
        self.reader.read_exact(&mut time2_buf);
        println!("request_num: {}", request_num);
        if request_num % 2 == 1 {
            // client so no third time stamp
            println!("Client packet");
            timestamp2 = u128::from_le_bytes(demo(time2_buf));
            println!("timestamp2: {}", timestamp2);
            assert_eq!(0 as u128, timestamp2); // should be 0
        }
        else {
            // server so third timestamp matters
            timestamp2 = u128::from_le_bytes(demo(time2_buf));
            assert_ne!(0 as u128, timestamp2);
        }

        let timestamp = u128::from_be_bytes(demo(time_buf));  // server's timestamp, we don't care
        //assert_ne!(0 as u128, timestamp);
        return Ok(Request{request_num, timestamp_send: timestamp, timestamp_receive: timestamp2});
              
    }
}

// client : Odd request num, timestamp_send, timestamp_receive = 0
// server: Even request num, timestamp_send, timestamp_receive != 0