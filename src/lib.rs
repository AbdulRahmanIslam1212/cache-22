pub enum Command<'a> {
    Set { key: &'a str, value: &'a str },
    Grab { key: &'a str },
    Replace { key: &'a str, value: &'a str },
    Delete { key: &'a str },
}

pub fn parse_request(req: &str) -> Result<Command<'_>, &'static str> {
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

pub fn match_result(result: Result<Command<'_>, &str>) {
    match result {
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
