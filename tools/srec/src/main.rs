use byteorder::{ByteOrder, LittleEndian};
use getopts::Options;
use std::io::{BufReader, BufWriter, Read, Write};
use std::{env, io, process};

const DEFAULT_OFFSET: u32 = 0;
// The original tool only output one word per line
const DEFAULT_WIDTH: u32 = 1;

struct Args {
    offset: u32,
    width: u32,
}

fn parse_u32(input: &str, field: &str, usage: &str) -> u32 {
    match input.parse::<u32>() {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("{} must be valid number\n{}", field, usage);
            process::exit(1);
        }
    }
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let prog = args[0].clone();
    let mut parsed_offset: u32 = DEFAULT_OFFSET;
    let mut parsed_width: u32 = DEFAULT_WIDTH;

    let mut opts = Options::new();
    opts.optopt("o", "offset", "output offset", "OFFSET");
    opts.optopt("w", "width", "output width in words", "WIDTH");
    opts.optflag("h", "help", "show cmd help");
    let usage = opts.short_usage(&prog);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}\n{}", f, usage);
            process::exit(1);
        }
    };
    if matches.opt_present("h") {
        println!("{}", usage);
        process::exit(0);
    }
    if let Some(opt) = matches.opt_str("o") {
        parsed_offset = parse_u32(&opt, "offset", &usage);
    }
    if let Some(opt) = matches.opt_str("w") {
        parsed_width = parse_u32(&opt, "width", &usage);
        if parsed_width < 1 {
            eprintln!("width cannot be zero\n{}", usage);
            process::exit(1);
        }
    }

    Args { offset: parsed_offset, width: parsed_width }
}

fn do_srec<R: Read, W: Write>(
    offset: u32,
    width: u32,
    input: &mut R,
    output: &mut W,
) -> io::Result<u32> {
    let mut pos = offset;

    output.write_all("/* http://srecord.sourceforge.net/ */\n".as_bytes())?;
    loop {
        for n in 0..width {
            let mut buf = [0u8; 4];
            let len = input.read(&mut buf)?;

            if len > 0 {
                let val = LittleEndian::read_u32(&buf);
                if n == 0 {
                    write!(output, "@{:08X} {:08X}", pos, val)?;
                } else {
                    write!(output, " {:08X}", val)?;
                }
            } else {
                if n != 0 {
                    writeln!(output)?;
                }
                return Ok(pos);
            }
            pos += 1;
        }
        writeln!(output)?;
    }
}

fn main() {
    let args = parse_args();
    let mut input = BufReader::new(io::stdin());
    let mut output = BufWriter::new(io::stdout());

    do_srec(args.offset, args.width, &mut input, &mut output).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    static EXAMPLE_RAW: &[u8] = include_bytes!("testdata/example.raw");
    static EXAMPLE_VMEM: &str = include_str!("testdata/example.vmem");
    const EXAMPLE_WIDTH: u32 = 7;

    #[test]
    fn basic_test() {
        let mut input = BufReader::new(EXAMPLE_RAW);
        let mut out_buf: Vec<u8> = Vec::with_capacity(EXAMPLE_VMEM.as_bytes().len());

        let final_off = do_srec(DEFAULT_OFFSET, EXAMPLE_WIDTH, &mut input, &mut out_buf).unwrap();
        assert_eq!(final_off as usize, EXAMPLE_RAW.len() / 4);

        let out_string = String::from_utf8(out_buf).unwrap();
        let mut out_lines = out_string.lines();
        let mut compare_lines = EXAMPLE_VMEM.lines();
        loop {
            match (out_lines.next(), compare_lines.next()) {
                (Some(a), Some(b)) => {
                    assert_eq!(a, b);
                }
                (Some(a), None) => {
                    panic!("extra line {}", a);
                }
                (None, Some(b)) => {
                    panic!("missing line {}", b);
                }
                (None, None) => {
                    break;
                }
            }
        }
    }
}
