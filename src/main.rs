use std::collections::HashSet;
use std::env;
use std::fs;

struct Record {
    timestamp: f64,
    signal: String,
    value: f64,
}

fn main() {
    let program_name = "frclog";
    let version = 2;
    // let supported_format = "CSV";

    let args = env::args().collect::<Vec<String>>();

    println!("{program_name} v{version}");
    // println!("Currently supported format(s): {supported_format}");
    // println!("{:?}", args);

    let command = match args.get(1) {
        Some(value) => value,
        None => {
            eprintln!("error: no command provided");
            return;
        }
    };

    let file = match args.get(2) {
        Some(value) => value,
        None => {
            eprintln!("error: no file provided");
            return;
        }
    };

    match command.as_str() {
        "info" => {
            println!("Running info on {file}...");
            let contents = fs::read_to_string(file);

            match contents {
                Ok(value) => {
                    println!("Successfully read {file}");

                    let mut records = Vec::new();

                    for line in value.lines().skip(1) {
                        match parse_record(line) {
                            Ok(rec) => {
                                records.push(rec);
                            }
                            Err(err) => {
                                eprintln!("{err}")
                            }
                        }
                    }

                    let mut signals = HashSet::new();

                    for rec in &records {
                        signals.insert(rec.signal.as_str());
                    }

                    println!("Total records: {}", value.lines().skip(1).count());
                    println!("Unique signals: {}", signals.len());
                    println!("Valid records: {}", records.len());

                    match (records.first(), records.last()) {
                        (Some(first), Some(last)) => {
                            println!("Duration: {}s", last.timestamp - first.timestamp);
                        }
                        _ => {
                            eprintln!("error: missing timestamps");
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                }
            }
        }
        "signals" => {
            println!("Listing signals in {file}...");
        }
        _ => {
            eprintln!("error: command '{command}' not recognized");
        }
    }
}

fn parse_record(line: &str) -> Result<Record, String> {
    let mut fields = line.split(',');

    let timestamp = fields
        .next()
        .ok_or(String::from("error: missing timestamp"))?
        .parse::<f64>()
        .map_err(|err| err.to_string())?;

    let signal = fields
        .next()
        .ok_or(String::from("error: missing signal"))?
        .to_string();

    let value = fields
        .next()
        .ok_or(String::from("error: missing value"))?
        .parse::<f64>()
        .map_err(|err| err.to_string())?;

    Ok(Record {
        timestamp,
        signal,
        value,
    })
}
