#![allow(unused_imports, unused_variables)]

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
    let dos_header = parse_dos_header(&mut reader)?;

    println!("e_magic:      {:#X}", dos_header.e_magic);
    println!("e_cblp:       {:#X}", dos_header.e_cblp);
    println!("e_cp:         {:#X}", dos_header.e_cp);
    println!("e_crlc:       {:#X}", dos_header.e_crlc);
    println!("e_cparhdr:    {:#X}", dos_header.e_cparhdr);
    println!("e_minalloc:   {:#X}", dos_header.e_minalloc);
    println!("e_maxalloc:   {:#X}", dos_header.e_maxalloc);
    println!("e_ss:         {:#X}", dos_header.e_ss);
    println!("e_sp:         {:#X}", dos_header.e_sp);
    println!("e_csum:       {:#X}", dos_header.e_csum);
    println!("e_ip:         {:#X}", dos_header.e_ip);
    println!("e_cs:         {:#X}", dos_header.e_cs);
    println!("e_lfarlc:     {:#X}", dos_header.e_lfarlc);
    println!("e_ovno:       {:#X}", dos_header.e_ovno);
    println!("e_oemid:      {:#X}", dos_header.e_oemid);
    println!("e_oeminfo:    {:#X}", dos_header.e_oeminfo);
    println!("e_lfanew:     {:#X}", dos_header.e_lfanew);

    Ok(())
}

pub fn parse_dos_header(
    reader: &mut std::io::BufReader<std::fs::File>,
) -> Result<IMAGE_DOS_HEADER, Box<dyn Error>> {
    let e_magic = reader.read_u16::<LittleEndian>()?;

    if e_magic != 0x5A4D {
        return Err("Invalid DOS header".into());
    }

    let e_cblp = reader.read_u16::<LittleEndian>()?;
    let e_cp = reader.read_u16::<LittleEndian>()?;
    let e_crlc = reader.read_u16::<LittleEndian>()?;
    let e_cparhdr = reader.read_u16::<LittleEndian>()?;
    let e_minalloc = reader.read_u16::<LittleEndian>()?;
    let e_maxalloc = reader.read_u16::<LittleEndian>()?;
    let e_ss = reader.read_u16::<LittleEndian>()?;
    let e_sp = reader.read_u16::<LittleEndian>()?;
    let e_csum = reader.read_u16::<LittleEndian>()?;
    let e_ip = reader.read_u16::<LittleEndian>()?;
    let e_cs = reader.read_u16::<LittleEndian>()?;
    let e_lfarlc = reader.read_u16::<LittleEndian>()?;
    let e_ovno = reader.read_u16::<LittleEndian>()?;
    let mut e_res: [u16; 4] = [0; 4];
    for i in 0..4 {
        e_res[i] = reader.read_u16::<LittleEndian>()?;
    }
    let e_oemid = reader.read_u16::<LittleEndian>()?;
    let e_oeminfo = reader.read_u16::<LittleEndian>()?;
    let mut e_res2: [u16; 10] = [0; 10];
    for i in 0..10 {
        e_res2[i] = reader.read_u16::<LittleEndian>()?;
    }
    let e_lfanew = reader.read_i32::<LittleEndian>()?;

    let dos_header = IMAGE_DOS_HEADER {
        e_magic,
        e_cblp,
        e_cp,
        e_crlc,
        e_cparhdr,
        e_minalloc,
        e_maxalloc,
        e_ss,
        e_sp,
        e_csum,
        e_ip,
        e_cs,
        e_lfarlc,
        e_ovno,
        e_res,
        e_oemid,
        e_oeminfo,
        e_res2,
        e_lfanew,
    };

    Ok(dos_header)
}
