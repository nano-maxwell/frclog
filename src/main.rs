mod reader;
mod record;
mod stats;

use crate::record::Record;
use crate::stats::calculate_signal_stats;
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

            let signals = unique_signals(&records);

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

            let signals = unique_signals(&records);

            let mut signals = signals.into_iter().collect::<Vec<&str>>();
            signals.sort();

            println!("Unique signals:");
            for signal in signals {
                println!("{signal}")
            }
        }
        "stats" => {
            let signal = match args.get(3) {
                Some(value) => value,
                None => {
                    eprintln!("error: no signal provided");
                    return;
                }
            };

            let records = match reader::read_records(file) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("{err}");
                    return;
                }
            };

            let signals = unique_signals(&records);

            if !signals.contains(signal.as_str()) {
                eprintln!("error: signal '{signal}' not found in {file}");
                return;
            }

            println!("Calculating stats for '{signal}' in {file}...");

            let filtered = filter_records(&records, signal);

            let stats = calculate_signal_stats(&filtered);

            println!("Stats for '{signal}':");
            println!("Samples: {}", stats.count);
            println!("Minimum: {:.3}", stats.min);
            println!("Maximum: {:.3}", stats.max);
            println!("Mean: {:.3}", stats.mean);
            println!("Standard Deviation: {:.3}", stats.std_dev);
        }
        _ => {
            eprintln!("error: command '{command}' not recognized");
        }
    }
}

fn unique_signals(records: &[Record]) -> HashSet<&str> {
    let mut signals = HashSet::new();

    for record in records {
        signals.insert(record.signal.as_str());
    }

    signals
}

fn filter_records<'a>(records: &'a [Record], signal: &str) -> Vec<&'a Record> {
    let mut filtered = Vec::new();

    for record in records {
        if record.signal.as_str() == signal {
            filtered.push(record);
        }
    }

    filtered
}
