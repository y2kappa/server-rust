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

    println!("Listening to {}:{}", host, port);

    let addr = format!("{}:{}", host, port).parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = listener.incoming().for_each(|socket| {
        // TODO: Process socket
        Ok(())
    })
    .map_err(|err| {
        // Handle error by printing to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:6142");

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