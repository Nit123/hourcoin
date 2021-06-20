use std::io;
use std::net::SocketAddr;

use structopt::StructOpt;


use protocol::{Protocol, Request, DEFAULT_SERVER_ADDR};



#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
struct Args {
    #[structopt(short, long, default_value = "1")]
    request_num: u16,
    /// Server destination address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();


    let req = Request::new_client_request(1);

    Protocol::connect(args.addr)
        .and_then(|mut client| {
            client.send_message(req)?;
            Ok(client)
        })
        .and_then(|mut client| client.read_message())
        .map(|resp| println!("server response: {} {} {}", resp.read_request_num(), resp.read_timestamp_send(), resp.read_timestamp_recieve()))
}