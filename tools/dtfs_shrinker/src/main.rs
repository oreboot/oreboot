use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::path::PathBuf;

lazy_static! {
    static ref HEX_REGEX: Regex = Regex::new(r"0x[0-9a-fA-F]+").unwrap();
}

fn find_hex(line: &str) -> &str {
    let caps = HEX_REGEX.captures(&line).expect("Invalid line format, expected a hex value");
    return caps.get(0).map_or("", |m| m.as_str());
}

fn parse_hex(val: &str) -> u32 {
    return u32::from_str_radix(val.trim_start_matches("0x"), 16).expect("Invalid hex value");
}

fn write_dtfs(dtfs_in: &str, dtfs_out: &str) -> std::io::Result<()> {
    // Round up to the nearest megabyte, but otherwise make the
    // payload area as small as needed to fit the payload itself.
    let payload = PathBuf::from(env::var("PAYLOAD_A").expect("PAYLOAD_A not in environment"));
    let payload_file_size : u32 = fs::metadata(payload)?.len().try_into().unwrap();
    let payload_area_size = (payload_file_size + 0x100000) & !0xFFFFF;

    let mut dtfs_file = fs::File::open(dtfs_in)?;
    let mut dtfs_file_new = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(dtfs_out)
        .expect("Failed to create new .dts file");

    // Pass #1: Preproccessing.
    // If we know the area of the payload ahead of time we can rewrite on the fly.
    let mut in_payload_area = false;
    let mut payload_area_index = 0;
    for line in io::BufReader::new(&dtfs_file).lines() {
        let line = line.unwrap();
        if line.contains(&"PAYLOAD_A") {
            in_payload_area = true;
        }
        if line.contains(&"};") {
            if in_payload_area {
                break;
            }
            payload_area_index += 1;
        }
    }


    let mut current_area_index = 0;
    let mut current_area_offset = 0;
    let mut current_area_size = 0;

    dtfs_file.seek(SeekFrom::Start(0))?;

    // Pass #2: Replace the size of the payload area and offsets of all subsequent areas.
    for line in io::BufReader::new(&dtfs_file).lines() {
        let line = line.unwrap();
        let mut line_to_print = line.clone();
        if line.contains(&"offset = ") {
            let hex = find_hex(&line);
            if current_area_index > payload_area_index {
                line_to_print = line.replace(hex, &format!("{:#x}", current_area_offset));
            }
        }
        else if line.contains(&"size = ") {
            let hex = find_hex(&line);
            current_area_size = parse_hex(hex);
            if current_area_index == payload_area_index {
                current_area_size = payload_area_size;
                line_to_print = line.replace(hex, &format!("{:#x}", current_area_size));
            }
        }
        else if line.contains(&"file = ") && line.contains(&"fixed-dtfs.dtb") {
            line_to_print = line.replace("fixed-dtfs.dtb", "fixed-dtfs-shrunk.dtb");
        }

        writeln!(dtfs_file_new, "{}", &line_to_print)?;

        if line.contains(&"};") {
            current_area_offset += current_area_size;
            current_area_index += 1;
        }
    }

    dtfs_file_new.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let dtfs_in = &args[1];
    let dtfs_out = &args[2];
    write_dtfs(dtfs_in, dtfs_out)?;

    Ok(())
}
