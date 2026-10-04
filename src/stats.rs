use crate::record::{Record, Value};

pub(crate) struct NumericStats {
    pub(crate) count: usize,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) mean: f64,
    pub(crate) std_dev: f64,
}

/// Calculates statistics for a non-empty slice of records with numeric values.
pub(crate) fn calculate_numeric_stats(records: &[&Record]) -> Result<NumericStats, String> {
    if records.is_empty() {
        return Err("error: cannot calculate numeric stats for empty records".to_string());
    }

    let mut count = 0;
    let mut sum = 0.0;
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for record in records {
        let value = match as_f64(&record.value) {
            Some(num) => num,
            None => {
                return Err(
                    "error: cannot calculate numeric stats for non-numeric values".to_string(),
                );
            }
        };

        sum += value;
        count += 1;
        min = f64::min(min, value);
        max = f64::max(max, value);
    }

    let mean = sum / count as f64;

    let mut squared_diff_sum = 0.0;

    for record in records {
        // Can unwrap as all values were already checked in the previous loop
        let value = as_f64(&record.value).unwrap();

        squared_diff_sum += (value - mean).powi(2);
    }

    let std_dev = (squared_diff_sum / count as f64).sqrt();

    Ok(NumericStats {
        count,
        min,
        max,
        mean,
        std_dev,
    })
}

fn as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Integer(num) => Some(*num as f64),
        Value::Float(num) => Some(*num),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_numeric_stats_for_float_values() {
        let records = [
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: Value::Float(1.0),
            },
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: Value::Float(10.0),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();

        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.mean, 5.5);
        assert_eq!(stats.std_dev, 4.5);
    }

    #[test]
    fn calculates_numeric_stats_for_integer_values() {
        let records = [
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: Value::Integer(1),
            },
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: Value::Integer(10),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();

        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.mean, 5.5);
        assert_eq!(stats.std_dev, 4.5);
    }

    #[test]
    fn calculates_numeric_stats_for_mixed_float_and_integer_values() {
        let records = [
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: Value::Float(1.0),
            },
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: Value::Integer(10),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();

        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.mean, 5.5);
        assert_eq!(stats.std_dev, 4.5);
    }

    #[test]
    fn calculates_numeric_stats_for_single_record() {
        let record = Record {
            timestamp: 1.0,
            signal: "voltage".to_string(),
            value: Value::Float(4.5),
        };

        let stats = calculate_numeric_stats(&[&record]).unwrap();

        assert_eq!(stats.count, 1);
        assert_eq!(stats.min, 4.5);
        assert_eq!(stats.max, 4.5);
        assert_eq!(stats.mean, 4.5);
        assert_eq!(stats.std_dev, 0.0);
    }

    #[test]
    fn calculates_numeric_stats_for_negative_values() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "temperature".to_string(),
                value: Value::Float(-10.0),
            },
            Record {
                timestamp: 2.0,
                signal: "temperature".to_string(),
                value: Value::Float(-2.0),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, -10.0);
        assert_eq!(stats.max, -2.0);
        assert_eq!(stats.mean, -6.0);
        assert_eq!(stats.std_dev, 4.0);
    }

    #[test]
    fn calculates_numeric_stats_when_values_are_identical() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "current".to_string(),
                value: Value::Float(3.0),
            },
            Record {
                timestamp: 2.0,
                signal: "current".to_string(),
                value: Value::Float(3.0),
            },
            Record {
                timestamp: 3.0,
                signal: "current".to_string(),
                value: Value::Float(3.0),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert_eq!(stats.count, 3);
        assert_eq!(stats.min, 3.0);
        assert_eq!(stats.max, 3.0);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.std_dev, 0.0);
    }

    #[test]
    fn calculates_fractional_mean() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "voltage".to_string(),
                value: Value::Float(0.1),
            },
            Record {
                timestamp: 2.0,
                signal: "voltage".to_string(),
                value: Value::Float(0.2),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert!((stats.mean - 0.15).abs() < 1e-12);
        assert!((stats.std_dev - 0.05).abs() < 1e-12);
    }

    #[test]
    fn calculates_non_integer_standard_deviation() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "test".to_string(),
                value: Value::Float(1.0),
            },
            Record {
                timestamp: 2.0,
                signal: "test".to_string(),
                value: Value::Float(2.0),
            },
            Record {
                timestamp: 3.0,
                signal: "test".to_string(),
                value: Value::Float(3.0),
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_numeric_stats(&record_refs).unwrap();

        assert!((stats.std_dev - 0.816496580927726).abs() < 1e-12);
    }

    #[test]
    fn rejects_non_numeric_values_in_numeric_calculation() {
        let record = Record {
            timestamp: 1.0,
            signal: "enabled".to_string(),
            value: Value::Boolean(true),
        };

        let result = calculate_numeric_stats(&[&record]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_records() {
        let records: [Record; 0] = [];

        let record_refs: Vec<&Record> = records.iter().collect();
        let result = calculate_numeric_stats(&record_refs);

        assert!(result.is_err());
    }
}
