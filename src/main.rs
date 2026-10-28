use cache22::{match_result, parse_request};
use std::{
    env,
    io::{BufRead, BufReader, Read},
    net::TcpListener,
    process,
    time::Duration,
};

struct Config<'a> {
    address: &'a String,
    port: u16,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Config<'a> {
        let address = &args[1];
        let port_binding = args[2].parse::<u16>();
        let port = match port_binding {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error parsing arguments: {e}");
                process::exit(1);
            }
        };

        Config { address, port }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let address = config.address;
    let port = config.port;

    let listener = TcpListener::bind(format!("{address}:{port}")).unwrap_or_else(|err| {
        eprintln!("Error binding to port: {err:?}");
        process::exit(1);
    });
    println!("Cache-22 Server currently active and listening...");
    run(listener);
}

fn run(listener: TcpListener) {
    const REQUEST_LIMIT: u64 = 8 * 1024;
    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            eprintln!("Error: failed to receive request");
            continue;
        };

        let read_timeout = stream.set_read_timeout(Some(Duration::from_secs(5)));
        match read_timeout {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error setting read timeout: {e}");
                continue;
            }
        }

        let mut buf = vec![];
        let mut buf_reader = BufReader::new(&stream);
        let mut reader = buf_reader.by_ref().take(REQUEST_LIMIT);
        let incoming_request = reader.read_until(b'\n', &mut buf);

        match incoming_request {
            Ok(count) => {
                if count == 0 {
                    eprintln!("client connected and left");
                    continue;
                } else if count >= REQUEST_LIMIT as usize && !buf.ends_with(b"\n") {
                    eprintln!("request too large");
                    continue;
                } else if !buf.ends_with(b"\n") {
                    eprintln!("request did not end in line terminator");
                    continue;
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        }

        let command_string = match String::from_utf8(buf) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };

        let parsed_request = parse_request(&command_string);
        match_result(parsed_request);
    }
}
