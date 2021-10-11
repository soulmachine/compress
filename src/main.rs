use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;

fn process_lines(
    buf_reader: &mut dyn std::io::BufRead,
    writer: &mut dyn std::io::Write,
) -> (i64, i64) {
    let mut total_lines = 0;
    let mut error_lines = 0;
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            total_lines += 1;
            if writeln!(writer, "{}", line).is_err() {
                error_lines += 1;
            }
        } else {
            panic!("Malformed file")
        }
    }
    writer.flush().unwrap();
    (error_lines, total_lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: compress <input_file> <output_file>");
        std::process::exit(1);
    }

    let input_file: &'static str = Box::leak(args[1].clone().into_boxed_str());
    let output_file: &'static str = Box::leak(args[2].clone().into_boxed_str());
    if !(output_file.ends_with(".gz") || output_file.ends_with(".xz")) {
        eprintln!("output_file must ends with .gz or .xz");
        std::process::exit(1);
    }

    let f_in =
        std::fs::File::open(input_file).unwrap_or_else(|_| panic!("{} does not exist", input_file));
    let mut buf_reader: Box<dyn std::io::BufRead> = if input_file.ends_with(".gz") {
        let d = GzDecoder::new(f_in);
        Box::new(std::io::BufReader::new(d))
    } else if input_file.ends_with(".xz") {
        let d = xz2::read::XzDecoder::new(f_in);
        Box::new(std::io::BufReader::new(d))
    } else {
        Box::new(std::io::BufReader::new(f_in))
    };

    let output_dir = std::path::Path::new(output_file).parent().unwrap();
    std::fs::create_dir_all(output_dir).unwrap();
    let f_out = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)
        .unwrap();
    let mut writer: Box<dyn std::io::Write> = if output_file.ends_with(".gz") {
        let encoder = GzEncoder::new(f_out, Compression::best());
        Box::new(std::io::BufWriter::new(encoder))
    } else if output_file.ends_with(".xz") {
        let e = xz2::write::XzEncoder::new(f_out, 9);
        Box::new(std::io::BufWriter::new(e))
    } else {
        Box::new(std::io::BufWriter::new(f_out))
    };

    let (error_lines, total_lines) = process_lines(buf_reader.as_mut(), writer.as_mut());
    if error_lines > 0 {
        eprintln!(
            "Found {} malformed lines out of {} lines",
            error_lines, total_lines,
        );
    }
}
