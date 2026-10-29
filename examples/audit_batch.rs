//! Binary UTF-8 batch transport for tools/audit-context.mjs, not an input method.
//! Each request and response is a little-endian u32 byte length then UTF-8 bytes.
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    let mut remaining = input.as_slice();
    let mut output = io::BufWriter::new(io::stdout().lock());
    while !remaining.is_empty() {
        let header = remaining
            .get(..4)
            .ok_or_else(|| io::Error::other("truncated length"))?;
        let size = u32::from_le_bytes(header.try_into().unwrap()) as usize;
        remaining = &remaining[4..];
        let raw = remaining
            .get(..size)
            .ok_or_else(|| io::Error::other("truncated input"))?;
        let raw = std::str::from_utf8(raw).map_err(io::Error::other)?;
        let converted = jong_bangla_core::transliterate(raw);
        let size = u32::try_from(converted.len()).map_err(io::Error::other)?;
        output.write_all(&size.to_le_bytes())?;
        output.write_all(converted.as_bytes())?;
        remaining = &remaining[raw.len()..];
    }
    output.flush()
}
