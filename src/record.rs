pub(crate) struct Record {
    pub(crate) timestamp: f64,
    pub(crate) signal: String,
    pub(crate) value: Value,
}

pub(crate) enum Value {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    Text(String),
}

impl Value {
    pub(crate) fn type_name(&self) -> &str {
        match self {
            Value::Float(_) | Value::Integer(_) => "Numeric",
            Value::Boolean(_) => "Boolean",
            Value::Text(_) => "Text",
        }
    }
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

    let value = parse_value(fields.next().ok_or(String::from("error: missing value"))?);

    Ok(Record {
        timestamp,
        signal,
        value,
    })
}

fn parse_value(value: &str) -> Value {
    if let Ok(boolean) = value.parse::<bool>() {
        return Value::Boolean(boolean);
    }

    if let Ok(int) = value.parse::<i64>() {
        return Value::Integer(int);
    }

    if let Ok(float) = value.parse::<f64>() {
        return Value::Float(float);
    }

    Value::Text(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_boolean_value() {
        let record = parse_record("1.0,enabled,true").unwrap();

        match &record.value {
            Value::Boolean(value) => assert!(*value),
            _ => panic!("expected boolean value"),
        }
    }

    #[test]
    fn parses_valid_record_with_integer_value() {
        let record = parse_record("1.0,piece_count,3").unwrap();

        match &record.value {
            Value::Integer(value) => assert_eq!(*value, 3),
            _ => panic!("expected integer value"),
        }
    }

    #[test]
    fn parses_valid_record_with_float_value() {
        let record = parse_record("1.0,voltage,5.0").unwrap();
        assert_eq!(record.timestamp, 1.0);
        assert_eq!(record.signal, "voltage");

        match &record.value {
            Value::Float(value) => assert_eq!(*value, 5.0),
            _ => panic!("expected float value"),
        }
    }

    #[test]
    fn parses_valid_record_with_text_value() {
        let record = parse_record("1.0,mode,teleop").unwrap();

        match &record.value {
            Value::Text(value) => assert_eq!(*value, "teleop"),
            _ => panic!("expected text value"),
        }
    }

    #[test]
    fn parses_empty_value() {
        let record = parse_record("1.0,voltage,").unwrap();

        match &record.value {
            Value::Text(value) => assert_eq!(*value, ""),
            _ => panic!("expected text value"),
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
    fn rejects_empty_line() {
        let result = parse_record("");
        assert!(result.is_err());
    }
}
