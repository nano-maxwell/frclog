mod reader;
mod record;

use std::collections::HashSet;
use std::env;

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

            let records = match reader::read_records(file) {
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

            let records = match reader::read_records(file) {
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
