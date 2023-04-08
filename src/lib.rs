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
    let file_size: u64 = contents.metadata()?.len();
    let mut reader = std::io::BufReader::new(contents);
    let mut pe = pe::Pe::new(&mut reader, file_size as usize);

    pe.parse_dos_header()?;
    pe.print_dos_header();

    pe.parse_dos_stub();
    pe.print_dos_stub();

    Ok(())
}
