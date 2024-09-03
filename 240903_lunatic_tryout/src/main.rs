use std::io::{Read, Write};

use lunatic::{net::{TcpListener, TcpStream}, spawn, spawn_link, Mailbox};

#[lunatic::main]
fn main(gmail: Mailbox<Vec<String>>) {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mailbox = gmail.monitorable();
    
    while let Ok((stream, _)) = listener.accept() {
        println!("found client");

        spawn!(|stream, mailbox| handle_connection(stream, mailbox.this.clone()));
    }
}

fn handle_connection(mut stream: TcpStream, mailbox: &Mailbox<Vec<String>>) {
    let mut buffer = [0; 1024];

    let stream_clone = stream.clone();

    let mailbox_handle = spawn_link!(|stream_clone, mailbox| handle_mailbox(stream_clone, mailbox));

    loop {
        match stream.read(&mut buffer) {
            Ok(_message) => {
                let string = String::from_utf8(buffer.to_vec()).unwrap();
                println!(
                    "Received message {}", string
                );

                mailbox_handle.send(buffer.to_vec())
            },
            _ => return
        }
    }
}

fn handle_mailbox(mut stream: TcpStream, mailbox: Mailbox<Vec<String>>) {
    loop {
        let request = mailbox.receive();

        stream.write(&request).unwrap();
    }
}