use std::fs;

pub(crate) fn read_records(file: &str) -> Result<Vec<crate::record::Record>, String> {
    let contents = fs::read_to_string(file).map_err(|err| err.to_string())?;

    let mut records = Vec::new();

    for line in contents.lines().skip(1) {
        records.push(crate::record::parse_record(line)?);
    }

    Ok(records)
}
