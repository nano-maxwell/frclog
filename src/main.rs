mod reader;
mod record;
mod stats;

use crate::record::{Record, Value};
use crate::stats::{calculate_boolean_stats, calculate_numeric_stats, calculate_text_stats};
use std::collections::{HashMap, HashSet};
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
        "signal" => {
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

            println!("Finding occurrences of '{signal}' in {file}...");

            let filtered = filter_records(&records, signal);

            if filtered.is_empty() {
                eprintln!("error: signal '{signal}' not found in {file}");
                return;
            }

            let signal_type = filtered[0].value.type_name();

            println!("Signal: {signal}");
            println!("Type: {signal_type}");
            println!("Samples: {}", filtered.len());
            println!();
            println!("{:<11} Value", "Timestamp");

            for record in filtered {
                match &record.value {
                    Value::Boolean(value) => {
                        println!("{:<11.3} {}", record.timestamp, value);
                    }
                    Value::Integer(value) => {
                        println!("{:<11.3} {}", record.timestamp, value);
                    }
                    Value::Float(value) => {
                        println!("{:<11.3} {:.3}", record.timestamp, value);
                    }
                    Value::Text(value) => {
                        println!("{:<11.3} {}", record.timestamp, value);
                    }
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

            let signal_width = signals
                .iter()
                .map(|signal| signal.len())
                .max()
                .unwrap_or(0)
                .max("Signal".len());

            let frequencies = signal_frequencies(&records);

            println!();
            println!("{:<width$}   Samples", "Signal", width = signal_width);

            for signal in signals {
                match frequencies.get(&signal) {
                    Some(value) => {
                        println!("{:<width$}   {}", signal, value, width = signal_width)
                    }
                    None => {
                        eprintln!(
                            "error: signal '{signal}' found in records but not in frequency hashmap"
                        )
                    }
                }
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

            let filtered = filter_records(&records, signal);

            if filtered.is_empty() {
                eprintln!("error: signal '{signal}' not found in {file}");
                return;
            }

            println!("Calculating stats for '{signal}' in {file}...");
            println!();

            match &filtered.first().unwrap().value {
                Value::Float(_) | Value::Integer(_) => match calculate_numeric_stats(&filtered) {
                    Ok(stats) => {
                        println!("Stats for '{signal}':");
                        println!("Samples: {}", stats.count);
                        println!("Minimum: {:.3}", stats.min);
                        println!("Maximum: {:.3}", stats.max);
                        println!("Mean: {:.3}", stats.mean);
                        println!("Standard Deviation: {:.3}", stats.std_dev);
                    }
                    Err(err) => {
                        eprintln!("{err}");
                    }
                },
                Value::Boolean(_) => {
                    match calculate_boolean_stats(&filtered) {
                        Ok(stats) => {
                            println!("Stats for '{signal}':");
                            println!("Samples: {}", stats.count);
                            println!("True Count: {}", stats.true_count);
                            println!("False Count: {}", stats.false_count);
                            println!("Percent True: {:.3}%", stats.percent_true);
                            println!("Transitions: {}", stats.transitions);
                        }
                        Err(err) => {
                            eprintln!("{err}");
                        }
                    };
                }
                Value::Text(_) => match calculate_text_stats(&filtered) {
                    Ok(stats) => {
                        println!("Stats for '{signal}':");
                        println!("Samples: {}", stats.count);
                        println!("Unique Values: {}", stats.unique_values);
                        println!("Transitions: {}", stats.transitions);
                    }
                    Err(err) => {
                        eprintln!("{err}");
                    }
                },
            };
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

fn signal_frequencies(records: &[Record]) -> HashMap<&str, i32> {
    let mut frequencies: HashMap<&str, i32> = HashMap::new();

    for record in records {
        let signal = record.signal.as_str();

        match frequencies.get(signal) {
            Some(value) => {
                frequencies.insert(signal, value + 1);
            }
            None => {
                frequencies.insert(signal, 1);
            }
        }
    }

    frequencies
}
