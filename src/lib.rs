use clap::Parser;
use core::panic;
use std::fmt::Write as _;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long = "fields", num_args = 1.., value_delimiter = ',')]
    fields: Vec<usize>,
    #[arg(short, long = "path")]
    path: PathBuf,
    #[arg(short, long = "delimiter")]
    delimiter: Option<String>,
}

pub struct Config {
    fields: Vec<usize>,
    path: PathBuf,
    delimiter: Option<String>,
}

impl Config {
    pub fn build(args: Cli) -> Config {
        let path = args.path;
        let delimiter = args.delimiter;

        Config {
            fields: args.fields,
            path,
            delimiter,
        }
    }
}

pub fn run(conf: Config) {
    let mut file = match open(conf.path) {
        Ok(a) => a,
        Err(_) => panic!("Could not open the file"),
    };
    cut(&mut file, conf.fields, conf.delimiter);
}

pub fn open(path: PathBuf) -> Result<BufReader<File>, std::io::Error> {
    let file = BufReader::new(File::open(path)?);
    Ok(file)
}

pub fn cut(file: &mut BufReader<File>, fields: Vec<usize>, delimiter: Option<String>) {
    // Used to store the lines of the file
    let mut buf = String::new();
    let mut line;
    // Used to store the extracted field values
    let mut field_buf = String::new();
    // Formatting the output, so it looks like Unix cut
    let width = 7;
    loop {
        line = file.read_line(&mut buf).unwrap();
        if line == 0 {
            break;
        }
        // let cuts: Vec<&str>;
        let cuts: Vec<&str> = match &delimiter {
            Some(d) => buf.split(d.as_str()).collect(),
            None => buf.split_whitespace().collect(),
        };
        dbg!(&cuts);
        // let mut blb = String::new();
        for field in &fields {
            write!(field_buf, "{:width$} ", cuts[*field - 1]).unwrap();
        }
        writeln!(field_buf).unwrap();
        buf.clear();
    }
    println!("{field_buf}");
}
