use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::{result, thread};

type Result<T> = result::Result<T, ()>;

struct Client {
    conn: Arc<TcpStream>,
}

enum Message {
    ClientConnected(Arc<TcpStream>),
    ClientDisconnected(Arc<TcpStream>),
    NewMessage {
        author: Arc<TcpStream>,
        bytes: Vec<u8>,
    },
}

const PORT: u16 = 6969;

fn server(messages: Receiver<Message>) -> Result<()> {
    let mut clients = HashMap::new();

    loop {
        let message = messages
            .recv()
            .expect("The server receiver is not hung up.");

        match message {
            Message::ClientConnected(author) => {
                let address = author
                    .peer_addr()
                    .expect("TODO: cache the peer address of the connection");

                clients.insert(
                    address.clone(),
                    Client {
                        conn: author.clone(),
                    },
                );
            }
            Message::ClientDisconnected(author) => {
                let address = author
                    .peer_addr()
                    .expect("TODO: cache the peer address of the connection");

                clients.remove(&address);
            }
            Message::NewMessage { author, bytes } => {
                let address = author
                    .peer_addr()
                    .expect("TODO: cache the peer address of the connection");

                for (addr, client) in clients.iter() {
                    if *addr != address {
                        let _ = client.conn.as_ref().write(&bytes);
                    }
                }
            }
        }
    }
}

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
            let _ = messages.send(Message::ClientDisconnected(stream.clone()));
        })?;

        println!("Info: Read into buffer");

        messages
            .send(Message::NewMessage {
                author: stream.clone(),
                bytes: buffer[0..n].to_vec(),
            })
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

    let (sender, receiver) = channel();

    println!("Info: Listening on {addr}");
    thread::spawn(|| server(receiver));

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
