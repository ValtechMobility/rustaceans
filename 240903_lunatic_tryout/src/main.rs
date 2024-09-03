use std::{collections::HashMap, io::{Read, Write}};

use lunatic::{net::{TcpListener, TcpStream}, spawn_link, Mailbox, Process};

use serde::{Deserialize, Serialize};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let coordinator_process = spawn_link!(|mailbox: Mailbox<Message>| coordinator(mailbox));

    let mut client_id = 1;
    while let Ok((stream, _)) = listener.accept() {
        println!("found client");
        spawn_link!(|coordinator_process, stream, client_id| handle_connection(coordinator_process.clone(), stream, client_id));
        client_id += 1;
    }
}

// Coordinator function
fn coordinator(mailbox: Mailbox<Message>) {
    // Store the list of clients
    let mut clients: HashMap<u32, Process<Message>> = HashMap::new();

    loop {
        let message = mailbox.receive();
        match message {
            // A new client joins
            Message::Join(client_id, client) => {
                clients.insert(client_id, client.clone());
                println!("New client joined: {}", client_id);
            }

            // Broadcast a message to all clients
            Message::Broadcast(msg) => {
                for client in clients.values() {
                    client.send(Message::Broadcast(msg.clone()));
                }
            }

            // A message received from a client
            Message::ClientMessage(msg, sender_client_id) => {
                println!("Received message from a client: {}", msg);
                // Broadcast the message to all clients
                for client_id in clients.keys() {
                    if client_id != &sender_client_id {
                        let client = clients.get(client_id);
                        if let Some(client) = client {
                            let msg = format!("Client {} said: {}", sender_client_id, msg);
                            client.send(Message::Broadcast(msg));
                        }
                    }
                }
            }
        }
    }
}

fn handle_connection(coordinator: Process<Message>, mut stream: TcpStream, client_id: u32) {
    let mut buffer = [0; 1024];

    let stream_clone = stream.clone();
    let mailbox_handle = spawn_link!(|stream_clone, mailbox: Mailbox<Message>| handle_mailbox(stream_clone, mailbox));

    coordinator.send(Message::Join(client_id, mailbox_handle));

    loop {
        match stream.read(&mut buffer) {
            Ok(_message) => {
                let msg = String::from_utf8(buffer.to_vec()).unwrap();
                coordinator.send(Message::ClientMessage(msg, client_id));
            },
            _ => return
        }
    }
}

fn handle_mailbox(mut stream: TcpStream, mailbox: Mailbox<Message>) {
    loop {
        let request = mailbox.receive();
        match request {
            Message::Broadcast(msg) => {
                stream.write(msg.as_bytes()).unwrap();
            }
            _=> {
                println!("Didn't receive a broadcast message");
            }
        }
    }
}

// Define the message types
#[derive(Serialize, Deserialize)]
enum Message {
    Join(u32, Process<Message>),          // A new client joins the chat
    Broadcast(String),               // A message to broadcast to all clients
    ClientMessage(String, u32), // A client sends a message
}