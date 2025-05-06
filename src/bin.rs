use gedcom::parser::Parser;
use gedcom::GedcomData;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => usage("Missing filename."),
        s if s > 2 => usage(&format!("Found more args than expected: {:?}", &args[1..])),
        _ => (),
    };

    let filename = &args[1];

    if filename == "--help" || filename == "-h" {
        usage("");
    }

    let data: GedcomData;

    let res = read_relative(&filename);
    if let Ok(contents) = res {
        let mut parser = Parser::new(contents.chars());
        data = parser.parse_record();

        println!("Parsing complete!");
        // println!("\n\n{:#?}", data);
        data.stats();
    } else if let Err(io_err) = res{
        exit_with_error(&format!("Could not open`{}` : {}", filename, io_err.to_string()));
    }
}

fn read_relative(path: &str) -> Result<String, std::io::Error> {
    let path_buf: PathBuf = PathBuf::from(path);
    let absolute_path: PathBuf = fs::canonicalize(path_buf)?;
    fs::read_to_string(absolute_path)
}

fn usage(msg: &str) {
    if !msg.is_empty() {
        println!("{}", msg);
    }
    println!("Usage: parse_gedcom ./path/to/gedcom.ged");
    std::process::exit(0x0100);
}

fn exit_with_error(msg: &str) {
    println!("Error! {}", msg);
    std::process::exit(0x1);
}
