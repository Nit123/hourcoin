use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};

use structopt::StructOpt;

use protocol::{Protocol, Request, DEFAULT_SERVER_ADDR};

#[derive(Debug, StructOpt)]
#[structopt(name = "server")]
struct Args {
    /// Service listening address
    #[structopt(long, default_value = DEFAULT_SERVER_ADDR, global = true)]
    addr: SocketAddr,
}

/// Given a TcpStream:
/// - Deserialize the request
/// - Handle the request
/// - Serialize and write the Response to the stream
fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Stream has peer_addr");
    let mut protocol = Protocol::with_stream(stream)?;

    let request = protocol.read_message().unwrap();
    eprintln!("Incoming {:?} [{}]", request, peer_addr);
    let resp = Request::new_server_request(request.read_request_num() + 1, request.read_timestamp_send());
    protocol.send_message(resp);
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::from_args();
    eprintln!("Starting server on '{}'", args.addr);

    let listener = TcpListener::bind(args.addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_connection(stream).map_err(|e| eprintln!("Error: {}", e))
            });
        }
    }
    Ok(())
}