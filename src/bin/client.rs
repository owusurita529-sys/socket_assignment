use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

const CUSTOMER_NAME: &str = "Client of Qby";
const DEFAULT_COUNTER: &str = "127.0.0.1:6000";

fn ask_for_number() -> i32 {
    loop {
        print!("[client] enter an integer between 1 and 100: ");
        io::stdout().flush().unwrap();

        let mut typed = String::new();
        io::stdin().read_line(&mut typed).unwrap();

        match typed.trim().parse::<i32>() {
            Ok(n) if (1..=100).contains(&n) => return n,
            _ => println!("[client] invalid input, try again"),
        }
    }
}

fn main() -> io::Result<()> {
    // Usage: client [address:port] [--shutdown]
    let cli_args: Vec<String> = env::args().skip(1).collect();
    let shutdown_mode = cli_args.iter().any(|a| a == "--shutdown");
    let counter_address = cli_args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(String::as_str)
        .unwrap_or(DEFAULT_COUNTER);

    // --shutdown sends an out-of-range value (0) to stop the server.
    let my_number = if shutdown_mode { 0 } else { ask_for_number() };

    println!("[client] connecting to {}", counter_address);
    let mut line_to_server = TcpStream::connect(counter_address)?;
    println!("[client] connected");

    let outgoing = format!("{}|{}\n", CUSTOMER_NAME, my_number);
    line_to_server.write_all(outgoing.as_bytes())?;
    println!("[client] sent name and number {}", my_number);

    if shutdown_mode {
        println!("[client] shutdown request sent, closing");
        return Ok(());
    }

    println!("[client] waiting for server reply...");
    let mut reply_reader = BufReader::new(&line_to_server);
    let mut reply = String::new();
    reply_reader.read_line(&mut reply)?;
    println!("[client] received raw reply: {:?}", reply.trim());

    let (server_name, server_number_text) = reply
        .trim()
        .split_once('|')
        .expect("server reply was malformed");
    let server_number: i32 = server_number_text
        .trim()
        .parse()
        .expect("server number was not an integer");

    println!("[client] client name : {}", CUSTOMER_NAME);
    println!("[client] server name : {}", server_name);
    println!("[client] client number = {}, server number = {}", my_number, server_number);
    println!("[client] sum = {}", my_number + server_number);

    println!("[client] socket released, done");
    Ok(())
}