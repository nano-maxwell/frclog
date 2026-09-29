use std::collections::HashSet;
use std::env;
use std::fs;

struct Record {
    timestamp: f64,
    signal: String,
    #[allow(dead_code)]
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

            let records = match read_records(file) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("{err}");
                    return;
                }
            };

            let mut signals = HashSet::new();

            for rec in &records {
                signals.insert(rec.signal.as_str());
            }

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
        "signals" => {
            println!("Listing signals in {file}...");

            let records = match read_records(file) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("{err}");
                    return;
                }
            };

            let mut signals = HashSet::new();

            for rec in &records {
                signals.insert(rec.signal.as_str());
            }

            let mut signals = signals.into_iter().collect::<Vec<&str>>();
            signals.sort();

            println!("Unique signals: ");
            for signal in signals {
                println!("{signal}")
            }
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

fn read_records(file: &str) -> Result<Vec<Record>, String> {
    let contents = fs::read_to_string(file).map_err(|err| err.to_string())?;

    println!("Successfully read {file}");

    let mut records = Vec::new();

    for line in contents.lines().skip(1) {
        records.push(parse_record(line)?);
    }

    Ok(records)
}
