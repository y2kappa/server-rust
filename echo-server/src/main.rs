extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;

fn main() {

    let mut config_file = File::open("src/config.json").unwrap();
    let mut config_data = String::new();
    config_file.read_to_string(&mut config_data).unwrap();
    let config_json : Value = serde_json::from_str(&config_data).unwrap();

    let host = String::from(config_json["host"].as_str().unwrap());
    let port = String::from(config_json["port"].as_str().unwrap());

    let addr = format!("{}:{}", host, port).parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = listener.incoming().for_each(|socket| {

        println!("Received connection {:?}", socket.local_addr());

        // split the socket stream into readable and writable parts
        // (tokio::io::ReadHalf<tokio::net::TcpStream>,
        // tokio::io::WriteHalf<tokio::net::TcpStream>)
        let (reader, writer) = socket.split();

        // copy bytes from the reader into the writer
        let amount = io::copy(reader, writer);

        let msg = amount.then(|result| {
            match result {
                Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                Err(e)             => println!("error: {}", e),
            }

            Ok(())
        });

        // spawn the task that handles the client connection socket on to the
        // tokio runtime. This means each client connection will be handled
        // concurrently
        tokio::spawn(msg);
        Ok(())

        })
    .map_err(|err| {
        // Handle error by printing to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("Server running on {}:{}", host, port);

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.

    tokio::run(server);
}