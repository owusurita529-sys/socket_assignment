use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const COUNTER_ADDRESS: &str = "0.0.0.0:6000";
const SHOPKEEPER_NAME: &str = "Server of Qby";
const SHOPKEEPER_PICK: i32 = 42;

/// Handles one visitor. Returns Ok(true) to keep the shop open,
/// Ok(false) when the visitor sent an out-of-range number (shutdown signal).
fn welcome_visitor(visitor: TcpStream) -> std::io::Result<bool> {
    let visitor_addr = visitor.peer_addr()?;
    println!("[server] connection opened from {}", visitor_addr);

    let mut note_reader = BufReader::new(&visitor);
    let mut incoming_note = String::new();
    note_reader.read_line(&mut incoming_note)?;
    println!("[server] received raw message: {:?}", incoming_note.trim());

    // Message format: "<name>|<number>\n"
    let (visitor_name, visitor_number_text) = match incoming_note.trim().split_once('|') {
        Some(parts) => parts,
        None => {
            println!("[server] malformed message, ignoring it");
            return Ok(true);
        }
    };

    let visitor_number: i32 = match visitor_number_text.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("[server] number was not an integer, ignoring it");
            return Ok(true);
        }
    };

    if !(1..=100).contains(&visitor_number) {
        println!(
            "[server] received out-of-range number {}, shutting down",
            visitor_number
        );
        return Ok(false);
    }

    println!("[server] client name : {}", visitor_name);
    println!("[server] server name : {}", SHOPKEEPER_NAME);
    println!(
        "[server] client number = {}, server number = {}, sum = {}",
        visitor_number,
        SHOPKEEPER_PICK,
        visitor_number + SHOPKEEPER_PICK
    );

    let reply = format!("{}|{}\n", SHOPKEEPER_NAME, SHOPKEEPER_PICK);
    (&visitor).write_all(reply.as_bytes())?;
    println!("[server] reply sent, closing this connection");
    Ok(true)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(COUNTER_ADDRESS)?;
    println!("[server] {} is listening on {}", SHOPKEEPER_NAME, COUNTER_ADDRESS);

    for arrival in listener.incoming() {
        let visitor = arrival?;
        if !welcome_visitor(visitor)? {
            break;
        }
    }

    // `listener` is dropped here, which releases the socket.
    println!("[server] socket released, goodbye");
    Ok(())
}