use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::{result, thread};

type Result<T> = result::Result<T, ()>;

enum Message {
    ClientConnected(Arc<TcpStream>),
    ClientDisonnected,
    NewMessage(Vec<u8>),
}

const PORT: u16 = 6969;

fn client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<()> {
    messages
        .send(Message::ClientConnected(stream.clone()))
        .map_err(|err| {
            eprintln!("Error: Couldn't send message to the server thread: {err}");
        })?;

    let mut buffer = [0; 1024];

    loop {
        let n = stream.as_ref().read(&mut buffer).map_err(|err| {
            eprintln!("Error: Cannot read message from client: {err}");
            let _ = messages.send(Message::ClientDisonnected);
        })?;

        println!("Info: Read into buffer");

        messages
            .send(Message::NewMessage(buffer[0..n].to_vec()))
            .map_err(|err| {
                eprintln!("Error: Couldn't send message to the server thread: {err}");
            })?;
    }
}

fn main() -> Result<()> {
    let addr = format!("0.0.0.0:{PORT}").parse::<SocketAddr>().unwrap();

    let listener = TcpListener::bind(addr).map_err(|err| {
        eprintln!("Error: Couldn't bind to {addr}: {err}");
    })?;

    let (sender, _receiver) = channel();

    println!("Info: Listening on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let sender = sender.clone();
                thread::spawn(|| client(stream, sender));
            }
            Err(err) => {
                eprintln!("Error: Couldn't accept connection {err}");
            }
        }
    }

    Ok(())
}
