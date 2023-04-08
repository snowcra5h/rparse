mod pe;

use std::{error::Error, fs};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path: String = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::File::open(config.file_path)?;
    let mut reader = std::io::BufReader::new(contents);
    let mut pe = pe::Pe::new();


    pe.parse_dos_header(&mut reader)?;
    pe.print_dos_header();

    Ok(())
}
