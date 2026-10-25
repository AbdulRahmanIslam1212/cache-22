use cache22::{match_result, parse_request};
use std::{
    env,
    io::{BufReader, prelude::*},
    net::TcpListener,
    process, thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = &args[1];
    let port: &u16 = &args[2].parse::<u16>().unwrap();

    let listener = TcpListener::bind(format!("{address}:{port}")).unwrap_or_else(|err| {
        eprintln!("Error binding to port: {err:?}");
        process::exit(1);
    });
    println!("Cache-22 Server currently active and listening...");

    let mut handles = vec![];

    for id in 1..5 {
        let handle = thread::spawn(move || {
            println!("Hello from thread #{id}");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap_or_else(|err| {
            eprintln!("Error: {err:?}");
            process::exit(1);
        });
    }

    //

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|err| {
            eprintln!("Error receiving requests: {err}");
            process::exit(1);
        });

        let buf_reader = BufReader::new(&stream);
        let incoming_request = buf_reader.lines().next().unwrap().unwrap();

        let parsed_request = parse_request(&incoming_request);
        match_result(parsed_request);
    }
}
