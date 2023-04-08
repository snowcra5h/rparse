#![allow(unused_imports, unused_variables)]

mod terminal;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{error::Error, fs};
use winapi::um::winnt::IMAGE_DOS_HEADER;

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
    let mut pe = pe::new();
    pe.parse_dos_header(&mut reader)?;
    pe.print_dos_header();

    Ok(())
}

pub struct pe {
    pub dos_header: IMAGE_DOS_HEADER,
}

impl pe {
    pub fn new() -> pe {
        pe {
            dos_header: IMAGE_DOS_HEADER {
                e_magic: 0,
                e_cblp: 0,
                e_cp: 0,
                e_crlc: 0,
                e_cparhdr: 0,
                e_minalloc: 0,
                e_maxalloc: 0,
                e_ss: 0,
                e_sp: 0,
                e_csum: 0,
                e_ip: 0,
                e_cs: 0,
                e_lfarlc: 0,
                e_ovno: 0,
                e_res: [0; 4],
                e_oemid: 0,
                e_oeminfo: 0,
                e_res2: [0; 10],
                e_lfanew: 0,
            },
        }
    }

    pub fn parse_dos_header(
        &mut self,
        reader: &mut std::io::BufReader<std::fs::File>,
    ) -> Result<(), Box<dyn Error>> {
        self.dos_header.e_magic = reader.read_u16::<LittleEndian>()?;

        if self.dos_header.e_magic != 0x5A4D {
            return Err("Invalid DOS header".into());
        }

        self.dos_header.e_cblp = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_cp = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_crlc = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_cparhdr = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_minalloc = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_maxalloc = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_ss = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_sp = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_csum = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_ip = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_cs = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_lfarlc = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_ovno = reader.read_u16::<LittleEndian>()?;
        for i in 0..4 {
            self.dos_header.e_res[i] = reader.read_u16::<LittleEndian>()?;
        }
        self.dos_header.e_oemid = reader.read_u16::<LittleEndian>()?;
        self.dos_header.e_oeminfo = reader.read_u16::<LittleEndian>()?;
        for i in 0..10 {
            self.dos_header.e_res2[i] = reader.read_u16::<LittleEndian>()?;
        }
        self.dos_header.e_lfanew = reader.read_i32::<LittleEndian>()?;

        Ok(())
    }

    pub fn print_dos_header(&self) {
        println!("DOS Header:");
        println!("  e_magic:      {:#06X}", self.dos_header.e_magic);
        println!("  e_cblp:       {:#06X}", self.dos_header.e_cblp);
        println!("  e_cp:         {:#06X}", self.dos_header.e_cp);
        println!("  e_crlc:       {:#06X}", self.dos_header.e_crlc);
        println!("  e_cparhdr:    {:#06X}", self.dos_header.e_cparhdr);
        println!("  e_minalloc:   {:#06X}", self.dos_header.e_minalloc);
        println!("  e_maxalloc:   {:#06X}", self.dos_header.e_maxalloc);
        println!("  e_ss:         {:#06X}", self.dos_header.e_ss);
        println!("  e_sp:         {:#06X}", self.dos_header.e_sp);
        println!("  e_csum:       {:#06X}", self.dos_header.e_csum);
        println!("  e_ip:         {:#06X}", self.dos_header.e_ip);
        println!("  e_cs:         {:#06X}", self.dos_header.e_cs);
        println!("  e_lfarlc:     {:#06X}", self.dos_header.e_lfarlc);
        println!("  e_ovno:       {:#06X}", self.dos_header.e_ovno);
        print!("  e_res:");
        for i in 0..4 {
            if i == 0 {
                print!("\t");
            } else {
                print!("{:#06X} ", self.dos_header.e_res[i]);
            }
        }
        println!("\n  e_oemid:      {:#06X}", self.dos_header.e_oemid);
        println!("  e_oeminfo:    {:#06X}", self.dos_header.e_oeminfo);
        print!("  e_res2:");
        for i in 0..10 {
            if i == 0 {
                print!("\t");
            } else {
                print!("{:#06X} ", self.dos_header.e_res2[i]);
            }
        }
        println!("\n  e_lfanew:     {:#010X}\n", self.dos_header.e_lfanew);
    }
}
