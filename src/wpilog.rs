pub(crate) struct WpilogHeader {
    #[allow(dead_code)]
    pub(crate) major_version: u8,
    #[allow(dead_code)]
    pub(crate) minor_version: u8,
    #[allow(dead_code)]
    pub(crate) extra_header: String,
}

pub(crate) struct StartControlRecord {
    #[allow(dead_code)]
    pub(crate) entry_id: u32,
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) entry_type: String,
    #[allow(dead_code)]
    pub(crate) metadata: String,
}

pub(crate) fn parse_file_header(file: &str, bytes: &[u8]) -> Result<WpilogHeader, String> {
    if bytes.len() < 12 {
        return Err(format!("error: '{file}' is missing required header bytes"));
    }

    // WPILOG files begin with a 6-byte ASCII string ("WPILOG").
    if &bytes[0..6] != b"WPILOG" {
        return Err(format!("error: '{file}' is not formatted correctly"));
    }

    // Bytes 6..8 store the version using 16 bits in little-endian order.
    let version_bytes: [u8; 2] = [bytes[6], bytes[7]];
    let version: u16 = u16::from_le_bytes(version_bytes);

    let major: u8 = (version >> 8) as u8;
    let minor: u8 = (version & 0xFF) as u8;

    // Bytes 8..12 store the length of the extra-header string, which is encoded in UTF-8.
    let extra_header_bytes: [u8; 4] = [bytes[8], bytes[9], bytes[10], bytes[11]];
    let extra_header_length: usize = u32::from_le_bytes(extra_header_bytes) as usize;
    let extra_header_end = 12 + extra_header_length;

    if bytes.len() < extra_header_end {
        return Err(format!("error: '{file}' is missing promised header bytes"));
    }

    let extra_header =
        std::str::from_utf8(&bytes[12..extra_header_end]).map_err(|err| err.to_string())?;

    Ok(WpilogHeader {
        major_version: major,
        minor_version: minor,
        extra_header: extra_header.to_string(),
    })
}

/// Parses a WPILOG record and returns the start position of the next record.
pub(crate) fn parse_record(file: &str, bytes: &[u8], record_start: usize) -> Result<usize, String> {
    if bytes.len() <= record_start {
        return Err(format!("error: '{file}' is missing records"));
    }

    let record_header_byte = bytes[record_start];

    println!("Record header byte: {record_header_byte}");
    println!("Record header byte (binary): {record_header_byte:08b}");

    // Bits 0-1 = entry ID byte count
    // Bits 2-3 = payload-size byte count
    // Bits 4-6 = timestamp byte count
    // Bit 7 = unused/spare

    let entry_id_length_code = record_header_byte & 0b00000011;
    let entry_id_length = entry_id_length_code + 1;

    let payload_size_length_code = (record_header_byte >> 2) & 0b00000011;
    let payload_size_length = payload_size_length_code + 1;

    let timestamp_length_code = (record_header_byte >> 4) & 0b00000111;
    let timestamp_length = timestamp_length_code + 1;

    println!("Entry ID length: {entry_id_length}");
    println!("Payload size length: {payload_size_length}");
    println!("Timestamp length: {timestamp_length}");

    let entry_id_start = record_start + 1;
    let entry_id_end = entry_id_start + entry_id_length as usize;

    if bytes.len() < entry_id_end {
        return Err(format!(
            "error: '{file}' is missing promised entry ID bytes"
        ));
    }

    let entry_id = decode_le_u64(&bytes[entry_id_start..entry_id_end])?;
    println!(
        "Entry ID raw bytes: {:?}",
        &bytes[entry_id_start..entry_id_end]
    );
    println!("Entry ID: {}", entry_id);

    let payload_size_start = entry_id_end;
    let payload_size_end = payload_size_start + payload_size_length as usize;

    if bytes.len() < payload_size_end {
        return Err(format!(
            "error: '{file}' is missing promised payload size bytes"
        ));
    }

    let payload_size = decode_le_u64(&bytes[payload_size_start..payload_size_end])?;

    println!(
        "Payload size raw bytes: {:?}",
        &bytes[payload_size_start..payload_size_end]
    );
    println!("Payload size: {payload_size}");

    let timestamp_start = payload_size_end;
    let timestamp_end = timestamp_start + timestamp_length as usize;

    if bytes.len() < timestamp_end {
        return Err(format!(
            "error: '{file}' is missing promised timestamp bytes"
        ));
    }

    println!(
        "Timestamp raw bytes: {:?}",
        &bytes[timestamp_start..timestamp_end]
    );

    let timestamp = decode_le_u64(&bytes[timestamp_start..timestamp_end])?;

    println!("Timestamp (microseconds): {}", timestamp);

    let payload_start = timestamp_end;
    let payload_end = payload_start + payload_size as usize;

    if bytes.len() < payload_end {
        return Err(format!("error: '{file}' is missing promised payload bytes"));
    }

    // If the entry ID is 0, then we're looking at a control record.
    // Any nonzero entry ID indicates a data record.
    if entry_id == 0 {
        if payload_size == 0 {
            return Err(format!(
                "error: '{file}' is missing promised control type byte"
            ));
        }

        if bytes[payload_start] == 0 {
            let _start_record =
                parse_start_control_record(file, bytes, payload_start, payload_end)?;
        } else {
            // TODO
        }
    }

    Ok(payload_end)
}

#[allow(dead_code)]
pub(crate) fn parse_start_control_record(
    file: &str,
    bytes: &[u8],
    payload_start: usize,
    payload_end: usize,
) -> Result<StartControlRecord, String> {
    // Structure of a Start control record payload:
    // 1 byte    control type = 0
    // 4 bytes   new entry ID
    // 4 bytes   entry name length
    // N bytes   entry name
    // 4 bytes   entry type length
    // N bytes   entry type
    // 4 bytes   metadata length
    // N bytes   metadata

    // + 1 to skip the control type (we already know that it's a Start record)
    let entry_id_start = payload_start + 1;
    let entry_id_end = entry_id_start + 4;

    if entry_id_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised start record entry ID bytes"
        ));
    }

    let entry_id = decode_le_u64(&bytes[entry_id_start..entry_id_end])? as u32;
    println!("Started entry ID: {entry_id}");

    let entry_name_length_start = entry_id_end;
    let entry_name_length_end = entry_name_length_start + 4;

    if entry_name_length_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised entry name length bytes"
        ));
    }

    let entry_name_length = decode_le_u64(&bytes[entry_name_length_start..entry_name_length_end])?;
    println!("Entry name length: {entry_name_length}");

    let entry_name_start = entry_name_length_end;
    let entry_name_end = entry_name_start + entry_name_length as usize;

    if entry_name_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised entry name bytes"
        ));
    }

    let entry_name =
        str::from_utf8(&bytes[entry_name_start..entry_name_end]).map_err(|err| err.to_string())?;
    println!("Entry name: {entry_name}");

    let entry_type_length_start = entry_name_end;
    let entry_type_length_end = entry_type_length_start + 4;

    if entry_type_length_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised entry type length bytes"
        ));
    }

    // For example, the length of "int64"
    let entry_type_length = decode_le_u64(&bytes[entry_type_length_start..entry_type_length_end])?;

    println!("Entry type length: {entry_type_length}");

    let entry_type_start = entry_type_length_end;
    let entry_type_end = entry_type_start + entry_type_length as usize;

    if entry_type_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised entry type bytes"
        ));
    }

    let entry_type =
        str::from_utf8(&bytes[entry_type_start..entry_type_end]).map_err(|err| err.to_string())?;
    println!("Entry type: {entry_type}");

    let metadata_length_start = entry_type_end;
    let metadata_length_end = metadata_length_start + 4;

    if metadata_length_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised metadata length bytes"
        ));
    }

    let metadata_length = decode_le_u64(&bytes[metadata_length_start..metadata_length_end])?;
    println!("Metadata length: {}", metadata_length);

    let metadata_start = metadata_length_end;
    let metadata_end = metadata_start + metadata_length as usize;

    if metadata_end > payload_end {
        return Err(format!(
            "error: '{file}' is missing promised metadata bytes"
        ));
    }

    let metadata =
        str::from_utf8(&bytes[metadata_start..metadata_end]).map_err(|err| err.to_string())?;

    println!("Metadata: {metadata}");

    if metadata_end != payload_end {
        return Err(format!(
            "error: '{file}' contains unexpected bytes after start record metadata"
        ));
    }

    Ok(StartControlRecord {
        entry_id,
        name: entry_name.to_string(),
        entry_type: entry_type.to_string(),
        metadata: metadata.to_string(),
    })
}

#[allow(dead_code)]
fn decode_le_u64(bytes: &[u8]) -> Result<u64, String> {
    if bytes.is_empty() {
        return Err("error: bytes cannot be empty".to_string());
    }

    if bytes.len() > 8 {
        return Err("error: cannot decode more than 8 bytes as u64".to_string());
    }

    let mut result = 0u64;
    let mut shift = 0;

    for byte in bytes {
        result |= (*byte as u64) << shift;
        shift += 8;
    }

    Ok(result)
}
