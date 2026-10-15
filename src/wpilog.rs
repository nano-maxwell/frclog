pub(crate) struct WpilogHeader {
    #[allow(dead_code)]
    major_version: u8,
    #[allow(dead_code)]
    minor_version: u8,
    #[allow(dead_code)]
    extra_header: String,
}

pub(crate) fn parse_header(file: &str, bytes: &[u8]) -> Result<WpilogHeader, String> {
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

#[allow(dead_code)]
fn read_little_endian(bytes: &[u8]) -> Result<u64, String> {
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
