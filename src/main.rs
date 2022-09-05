use std::path::PathBuf;
use std::fs;
use secret_message::morse::*;
use clap::{ArgGroup, Parser, ValueEnum};
use regex;

/// Enum for the two possible actions: encoding into morse or decoding morse
#[derive(Copy, Clone, PartialEq, Debug,ValueEnum)]
enum Mode {
    Encode,
    Decode
}

/// A simple program for encoding to and decoding from morse code
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["message", "source-file"])
    )
)]
struct Args {
    /// The mode specifies if the program should encode or decode
    #[clap(arg_enum, value_parser)]
    mode: Mode,
    /// If given the program will operate on the message input directly on the command line
    #[clap(short, long, value_parser)]
    message: Option<String>,
    /// If given the program will read the message input from the file at the given path
    #[clap(short='c', long, value_parser)]
    source_file: Option<PathBuf>,
    /// If given the program will try to write it into the file at this path. If the file does not
    /// exist the program will attempt to create it.
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>
}

fn read_file(path: &PathBuf) -> String {
    if !path.try_exists().expect("Could not check if path exists") {
        panic!("Cannot read from {:?} as it does not exist", path)
    }
    String::from_utf8(
        fs::read(path).expect("Unable to read file")
    ).expect("Unable to decode bytes")
}

fn write_to_file(path: &PathBuf, msg: &str) {
    fs::write(path, msg).expect("Unable to write to file");
}

fn main() {
    let args = Args::parse();
    let msg = if args.message.is_some() {
        args.message.unwrap()
    } else {
        let path: &PathBuf = &args.source_file.unwrap();
        read_file(path)
    };
    let regex: regex::Regex = match args.mode {
        Mode::Encode => regex::Regex::new(r"^[a-zA-Z0-9\s]*$").expect("Unable to create regex"), // encode
                                                                                                 // allowed
                                                                                                 // chars
        Mode::Decode => regex::Regex::new(r"^[·―\s]*$").expect("Unable to create regex") // Decode
                                                                                         // allowed
                                                                                         // chars
    };
    if !regex.is_match(&msg) {
        eprintln!("The given message contains unallowed characters");
        std::process::exit(1)
    }
    let msg = msg.to_lowercase();
    let translated = match args.mode {
        Mode::Encode => translate_to_morse(msg.trim()),
        Mode::Decode => translate_from_morse(msg.trim()),
    };
    if let Some(output) = args.output {
        write_to_file(&output, translated.trim())
    } else {
        println!("{}", translated.trim());
    }
}
