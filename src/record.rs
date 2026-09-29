pub(crate) struct Record {
    pub(crate) timestamp: f64,
    pub(crate) signal: String,
    #[allow(dead_code)]
    pub(crate) value: f64,
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
