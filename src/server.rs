use std::net::TcpListener;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let addr = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr).map_err(|err| {
        eprintln!("Error: Couldn't bind to {addr}: {err}");
    })?;

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Connection accepted!");
            }
            Err(err) => {
                eprintln!("Error: Couldn't accept connection {err}");
            }
        }
    }

    Ok(())
}
