pub(crate) struct Record {
    pub(crate) timestamp: f64,
    pub(crate) signal: String,
    pub(crate) value: Value,
}

pub(crate) enum Value {
    Float(f64),
    #[allow(dead_code)]
    Integer(i64),
    #[allow(dead_code)]
    Boolean(bool),
    #[allow(dead_code)]
    Text(String),
}

pub(crate) fn parse_record(line: &str) -> Result<Record, String> {
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

    if signal.is_empty() {
        return Err(String::from("error: empty signal name"));
    }

    let value = fields
        .next()
        .ok_or(String::from("error: missing value"))?
        .parse::<f64>()
        .map_err(|err| err.to_string())?;

    Ok(Record {
        timestamp,
        signal,
        value: Value::Float(value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let record = parse_record("1.0,voltage,5.0").unwrap();
        assert_eq!(record.timestamp, 1.0);
        assert_eq!(record.signal, "voltage");

        match &record.value {
            Value::Float(value) => assert_eq!(*value, 5.0),
            _ => panic!("expected float value"),
        }
    }

    #[test]
    fn rejects_invalid_record() {
        let result = parse_record("1.0,voltage");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_timestamp() {
        let result = parse_record("not_a_number,voltage,5.0");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_value() {
        let result = parse_record("1.0,voltage,not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_timestamp() {
        let result = parse_record(",voltage,5.0");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_signal() {
        let result = parse_record("1.0,,5.0");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_value() {
        let result = parse_record("1.0,voltage,");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_line() {
        let result = parse_record("");
        assert!(result.is_err());
    }
}
