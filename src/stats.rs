use crate::record::Record;

pub(crate) struct SignalStats {
    pub(crate) count: usize,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) mean: f64,
    pub(crate) std_dev: f64,
}

/// Calculates statistics for a non-empty slice of records.
pub(crate) fn calculate_signal_stats(records: &[&Record]) -> SignalStats {
    let mut count = 0;
    let mut sum = 0.0;
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for record in records {
        sum += record.value;
        count += 1;
        min = f64::min(min, record.value);
        max = f64::max(max, record.value);
    }

    let mean = sum / count as f64;

    let mut squared_diff_sum = 0.0;

    for record in records {
        squared_diff_sum += (record.value - mean).powi(2);
    }

    let std_dev = (squared_diff_sum / count as f64).sqrt();

    SignalStats {
        count,
        min,
        max,
        mean,
        std_dev,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_stats_for_multiple_records() {
        let records = [
            Record {
                value: 1.0,
                signal: "elevator_current".to_string(),
                timestamp: 2.0,
            },
            Record {
                timestamp: 2.0,
                signal: "elevator_current".to_string(),
                value: 10.0,
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();

        let stats = calculate_signal_stats(&record_refs);

        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.mean, 5.5);
        assert_eq!(stats.std_dev, 4.5);
    }

    #[test]
    fn calculates_stats_for_single_record() {
        let record = Record {
            timestamp: 1.0,
            signal: "voltage".to_string(),
            value: 4.5,
        };

        let stats = calculate_signal_stats(&[&record]);

        assert_eq!(stats.count, 1);
        assert_eq!(stats.min, 4.5);
        assert_eq!(stats.max, 4.5);
        assert_eq!(stats.mean, 4.5);
        assert_eq!(stats.std_dev, 0.0);
    }

    #[test]
    fn calculates_stats_for_negative_values() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "temperature".to_string(),
                value: -10.0,
            },
            Record {
                timestamp: 2.0,
                signal: "temperature".to_string(),
                value: -2.0,
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_signal_stats(&record_refs);

        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, -10.0);
        assert_eq!(stats.max, -2.0);
        assert_eq!(stats.mean, -6.0);
        assert_eq!(stats.std_dev, 4.0);
    }

    #[test]
    fn calculates_stats_when_values_are_identical() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "current".to_string(),
                value: 3.0,
            },
            Record {
                timestamp: 2.0,
                signal: "current".to_string(),
                value: 3.0,
            },
            Record {
                timestamp: 3.0,
                signal: "current".to_string(),
                value: 3.0,
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_signal_stats(&record_refs);

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
                value: 0.1,
            },
            Record {
                timestamp: 2.0,
                signal: "voltage".to_string(),
                value: 0.2,
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_signal_stats(&record_refs);

        assert!((stats.mean - 0.15).abs() < 1e-12);
        assert!((stats.std_dev - 0.05).abs() < 1e-12);
    }

    #[test]
    fn calculates_non_integer_standard_deviation() {
        let records = [
            Record {
                timestamp: 1.0,
                signal: "test".to_string(),
                value: 1.0,
            },
            Record {
                timestamp: 2.0,
                signal: "test".to_string(),
                value: 2.0,
            },
            Record {
                timestamp: 3.0,
                signal: "test".to_string(),
                value: 3.0,
            },
        ];

        let record_refs: Vec<&Record> = records.iter().collect();
        let stats = calculate_signal_stats(&record_refs);

        assert!((stats.std_dev - 0.816496580927726).abs() < 1e-12);
    }
}
