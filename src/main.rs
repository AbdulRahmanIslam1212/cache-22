use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    process, thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1212").unwrap_or_else(|err| {
        eprintln!("Error binding to port: {err:?}");
        process::exit(1);
    });
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

        match parsed_request {
            Ok(result) => match result {
                Command::Grab { key } => {
                    println!("Request:\n\nCommand: GRAB, key: {key}");
                }
                Command::Set { key, value } => {
                    println!("Request:\n\nCommand: SET, key: {key}, value: {value}");
                }
                Command::Replace { key, value } => {
                    println!("Request:\n\nCommand: REPLACE, key: {key}, value: {value}");
                }
                Command::Delete { key } => {
                    println!("Request:\n\nCommand: DELETE, key: {key}");
                }
            },
            Err(err) => println!("{err}"),
        }
    }
}
#[derive(Debug)]
enum Command<'a> {
    Set { key: &'a str, value: &'a str },
    Grab { key: &'a str },
    Replace { key: &'a str, value: &'a str },
    Delete { key: &'a str },
}

fn parse_request(req: &str) -> Result<Command<'_>, &'static str> {
    // Split the string into whitespace-separated parts
    let mut parts = req.split_whitespace();

    // Extract the keyword (the first word)
    let keyword = parts.next().ok_or("Empty request")?;

    match keyword {
        "SET" => {
            let key = parts.next().ok_or("SET missing <key>")?;
            let value = parts.next().ok_or("SET missing <value>")?;

            // Check if there are extra unexpected arguments
            if parts.next().is_some() {
                return Err("SET has too many arguments");
            }

            Ok(Command::Set { key, value })
        }
        "GRAB" => {
            let key = parts.next().ok_or("GRAB missing <key>")?;
            if parts.next().is_some() {
                return Err("GRAB has too many arguments");
            }
            Ok(Command::Grab { key })
        }
        "REPLACE" => {
            let key = parts.next().ok_or("REPLACE missing <key>")?;
            let value = parts.next().ok_or("REPLACE missing <value>")?;
            if parts.next().is_some() {
                return Err("REPLACE has too many arguments");
            }
            Ok(Command::Replace { key, value })
        }
        "DELETE" => {
            let key = parts.next().ok_or("DELETE missing <key>")?;
            if parts.next().is_some() {
                return Err("DELETE has too many arguments");
            }
            Ok(Command::Delete { key })
        }
        _ => Err("Invalid request: must start with SET, GRAB, REPLACE, or DELETE"),
    }
}
