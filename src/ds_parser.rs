#![allow(
    unused_variables,
    unreachable_code,
    unused_imports,
    dead_code,
    unused_parens,
    deprecated
)]

use chrono::{TimeZone, Utc};
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH}; // Chrono is a popular date-time library in Rust

// mod byte_buffer;
// mod ds_result;
extern crate chrono; // This line declares the chrono crate
extern crate color_print;
// use chrono::TimeZone;
use crate::byte_buffer::ByteBuffer;
use crate::ds_result::ResultData;
use color_print::{cprint, cprintln};

use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Error, Read};
use std::{error, io, time};

enum ParsedValue {
    // Bool(bool),
    // Short(u32),
    // Long(u32),
    // Comp(u64),
    // Dutch(u64),
    // Type(String),
    // Blob(Vec<u8>),
    // Ustr(String),
    Intx(u64),
    Unrecognized,
}
fn parse_header(buffer: &mut ByteBuffer) -> io::Result<(u32, u32)> {
    let alignment: u32 = buffer.read_uint32()?;
    assert_eq!(alignment, 0x00000001);
    let magic: u32 = buffer.read_uint32()?;
    assert_eq!(magic, 0x42756431);

    let alloc_offset: u32 = 0x4 + buffer.read_uint32()?;
    let alloc_len: u32 = buffer.read_uint32()?;
    let alloc_offset_repeat: u32 = 0x4 + buffer.read_uint32()?;
    assert_eq!(alloc_offset_repeat, alloc_offset);

    Ok((alloc_offset, alloc_len))
}

fn parse_allocator(buffer: &mut ByteBuffer, alloc_offset: u32) -> io::Result<(u32, Vec<u32>)> {
    buffer.reset();
    buffer.skip(alloc_offset as u64);
    let num_offsets = buffer.read_uint32()?;
    let second: u32 = buffer.read_uint32()?;
    assert_eq!(second, 0);

    let offsets = (0..num_offsets)
        .map(|_| buffer.read_uint32().unwrap())
        .collect::<Vec<u32>>();
    buffer.reset();
    buffer.skip((alloc_offset + 0x408) as u64);
    let mut directory: HashMap<String, u32> = HashMap::new();
    let num_keys: u32 = buffer.read_uint32()?;
    for _ in 0..num_keys {
        let key_len: u8 = buffer.read_uint8()?;
        let key = buffer.read_string(key_len as usize)?;
        let value = buffer.read_uint32()?;

        directory.insert(key, value);
    }
    if !directory.contains_key("DSDB") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Key 'DSDB' not found in table of contents",
        ));
    }
    // cprintln!("<bold>- Directory:</bold> <cyan>{:?}</cyan>", directory);
    let master_id = directory["DSDB"];

    Ok((master_id, offsets))
}

fn align_and_adjust_cursor(offset_and_size: u32) -> u32 {
    const MASK_32_BYTE_ALIGN: u32 = !0x1f; // Mask to align to 32-byte boundary
    const ADDITIONAL_OFFSET: u32 = 0x4; // Additional offset to add after alignment

    // Align to 32-byte boundary by clearing the least significant 5 bits
    let aligned_offset = offset_and_size & MASK_32_BYTE_ALIGN;

    // Add an additional 4-byte offset
    let adjusted_cursor = aligned_offset + ADDITIONAL_OFFSET;

    adjusted_cursor
}

fn parse_master_node(buffer: &mut ByteBuffer, offset_and_size: u32) -> io::Result<(u32)> {
    let next_cursor = align_and_adjust_cursor(offset_and_size);
    buffer.reset();
    buffer.skip(next_cursor as u64);
    let root_id = buffer.read_uint32()?;
    let tree_height = buffer.read_uint32()?;
    let num_records = buffer.read_uint32()?;
    let num_nodes = buffer.read_uint32()?;
    let fifth = buffer.read_uint32()?;
    assert_eq!(fifth, 0x1000);
    Ok((root_id))
}

fn parse_blob_to_datetime(data: &[u8]) -> u64 {
    let bytes: [u8; 8] = data
        .try_into()
        .expect("Failed to convert data to byte array");
    let hex_str = hex::encode(bytes);
    let rearranged = format!(
        "{}{}{}{}{}{}{}{}",
        &hex_str[14..16],
        &hex_str[12..14],
        &hex_str[10..12],
        &hex_str[8..10],
        &hex_str[6..8],
        &hex_str[4..6],
        &hex_str[2..4],
        &hex_str[0..2]
    );
    let rearranged_bytes = hex::decode(rearranged).expect("Failed to decode hex");
    let num = f64::from_be_bytes(
        rearranged_bytes
            .try_into()
            .expect("Incorrect length for f64"),
    );
    // Calculate the datetime from the timestamp
    let mac_epoch_offset = 978307200; // seconds from UNIX_EPOCH to Mac epoch
    let timestamp_secs = num + mac_epoch_offset as f64;
    let datetime = Utc.timestamp(timestamp_secs as i64, 0);
    return datetime.timestamp_millis() as u64;
}

fn vec_to_u64_be(bytes: Vec<u8>) -> Result<u64, &'static str> {
    if bytes.len() != 8 {
        // Ensure there are exactly 8 bytes, as required for a u64
        return Err("Input Vec<u8> must contain exactly 8 bytes for conversion to u64.");
    }

    // Convert Vec<u8> to an array of 8 elements
    let bytes_array: [u8; 8] = bytes
        .try_into()
        .map_err(|_| "Failed to convert Vec<u8> to [u8; 8]")?;

    // Use from_be_bytes to convert the byte array to a u64
    Ok(u64::from_be_bytes(bytes_array))
}

fn parse_datatype(
    data_type: &str,
    field: &str,
    buffer: &mut ByteBuffer,
) -> io::Result<(ParsedValue)> {
    match data_type {
        "bool" => {
            let value = buffer.read_uint8()? != 0;
            Ok(ParsedValue::Unrecognized)
        }
        "shor" | "long" => {
            let value = buffer.read_uint32()?;
            Ok(ParsedValue::Unrecognized)
        }
        "comp" | "dutc" => {
            let value = buffer.read_uint64()?;
            if (field == "ph1S" || field == "phyS") {
                Ok(ParsedValue::Intx(value))
            } else {
                Ok(ParsedValue::Unrecognized)
            }
        }
        "type" => {
            let value = buffer.read_string(4)?;
            Ok(ParsedValue::Unrecognized)
        }
        "blob" => {
            let data_length = buffer.read_uint32()?;
            let value = buffer.read_uint8_array(data_length as usize)?;
            // only for field == "moDD" || field == "modD":
            if field == "moDD" || field == "modD" {
                let timestamp = parse_blob_to_datetime(&value);
                Ok(ParsedValue::Intx(timestamp))
            } else {
                Ok(ParsedValue::Unrecognized)
            }
        }
        "ustr" => {
            let data_length = buffer.read_uint32()?;
            let value = buffer
                .read_string_utf16_be(data_length as usize * 2)
                .unwrap();
            Ok(ParsedValue::Unrecognized)
        }
        // throw here
        _ => Ok(ParsedValue::Unrecognized),
    }
}

fn parse_b_tree(
    buffer: &mut ByteBuffer,
    result_data: &mut ResultData,
    offsets: Vec<u32>,
    node_id: u32,
    depth: u32,
    // rtn: HashMap<String, HashMap<String, Vec<u8>>>,
) -> io::Result<()> {
    // println!(">>>>>>>>>>>>>>>parse_b_tree {}", depth);
    let offset_and_size = offsets[node_id as usize];
    let next_cursor = align_and_adjust_cursor(offset_and_size);
    buffer.reset();
    buffer.skip(next_cursor as u64);

    let next_id = buffer.read_uint32()?;
    let num_records = buffer.read_uint32()?;

    for _ in 0..num_records - 1 {
        if next_id != 0 {
            let child_id = buffer.read_uint32()?;
            let current_cursor = buffer.byte_offset();
            parse_b_tree(buffer, result_data, offsets.clone(), child_id, depth + 1)?;
            buffer.reset();
            buffer.skip(current_cursor);
        }
        let name_length = buffer.read_uint32()?;
        if name_length > buffer.bytes_remaining() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough bytes",
            ));
        }
        let name = buffer
            .read_string_utf16_be((name_length as usize) * 2)
            .unwrap();
        let field = buffer.read_string(4)?;
        let dtype = buffer.read_string(4)?;

        match parse_datatype(&dtype, &field, buffer) {
            Ok(ParsedValue::Intx(value)) => {
                result_data.add_record(&name, &field, value);
                // cprintln!("<bold> - {}: <green>{}</green></bold>", field, value);
            }
            Ok(ParsedValue::Unrecognized) => {
                // cprintln!("<bold> - value: <red>Unrecognized</red></bold>");
            }
            Err(e) => {
                cprintln!("<bold> - error: <red>{}</red></bold>", e);
            }
        }
    }
    Ok(())
}

pub fn get_ds_cache(path: &Path) -> io::Result<ResultData> {
    let ds_store_path = if path.is_dir() {
        path.join(".DS_Store")
    } else {
        path.to_path_buf()
    };

    if !ds_store_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            ".DS_Store not found",
        ));
    }
    let mut file = std::fs::File::open(ds_store_path)?;
    let mut data = Vec::new();
    let mut result_data = ResultData::new();

    file.read_to_end(&mut data)?;
    let mut buffer = ByteBuffer::new(data);
    let (alloc_offset, alloc_len) = parse_header(&mut buffer)?;
    let (master_id, offsets) = parse_allocator(&mut buffer, alloc_offset)?;
    let root_id = parse_master_node(&mut buffer, offsets[master_id as usize])?;
    parse_b_tree(&mut buffer, &mut result_data, offsets, root_id, 0)?;
    // print results:
    cprintln!("<bold>Results:</bold>");
    for (key, value) in result_data.get_files().iter() {
        cprintln!("<bold> - {}: <green>{:?}</green></bold>", key, value);
    }
    Ok(result_data)
}

// fn main() -> io::Result<()> {
//     let args: Vec<String> = std::env::args().collect();
//     if args.len() < 2 {
//         return Err(io::Error::new(
//             io::ErrorKind::InvalidInput,
//             "No path provided",
//         ));
//     }

//     let path = std::path::Path::new(&args[1]);
//     get_ds_cache(path)?;

//     Ok(())
// }
