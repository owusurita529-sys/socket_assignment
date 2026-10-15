 # Socket Programming Assignment (Rust)

A TCP client and server written in Rust using only `std::net`.

## How it works

- The client asks for an integer between 1 and 100, then sends `name|number` (plus a newline) to the server.
- The server prints both names, both numbers, and their sum, then replies with its own name and number (42).
- The client prints both names, both numbers, and the sum, then exits.
- If the server receives a number outside 1 to 100, it releases its socket and shuts down.

## Run it

Terminal 1:

```bash
cargo run --bin server
```

Terminal 2:

```bash
cargo run --bin client
```

Shut the server down:

```bash
cargo run --bin client -- --shutdown
```

Connect to another server:

```bash
cargo run --bin client -- IP_ADDRESS:PORT
```

## Screenshots

See the `screenshots/` folder for the normal run and the shutdown run.