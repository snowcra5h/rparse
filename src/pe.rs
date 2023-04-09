use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    error::Error,
    io::{Cursor, Read},
};
use winapi::um::winnt::{IMAGE_DOS_HEADER, IMAGE_NT_HEADERS};

pub struct Pe {
    cursor: Cursor<Vec<u8>>,
    pub dos_header: IMAGE_DOS_HEADER,
    pub dos_stub: Vec<u8>,
    pub nt_header: IMAGE_NT_HEADERS,
}

impl Pe {
    pub fn new(reader: &mut std::io::BufReader<std::fs::File>, file_size: usize) -> Pe {
        Pe {
            cursor: {
                let mut buffer = vec![0; file_size];
                reader.read_exact(&mut buffer).unwrap();
                let cursor = Cursor::new(buffer);
                cursor
            },
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
            dos_stub: Vec::new(),
            nt_header: winapi::um::winnt::IMAGE_NT_HEADERS {
                Signature: 0,
                FileHeader: winapi::um::winnt::IMAGE_FILE_HEADER {
                    Machine: 0,
                    NumberOfSections: 0,
                    TimeDateStamp: 0,
                    PointerToSymbolTable: 0,
                    NumberOfSymbols: 0,
                    SizeOfOptionalHeader: 0,
                    Characteristics: 0,
                },
                OptionalHeader: winapi::um::winnt::IMAGE_OPTIONAL_HEADER64 {
                    Magic: 0,
                    MajorLinkerVersion: 0,
                    MinorLinkerVersion: 0,
                    SizeOfCode: 0,
                    SizeOfInitializedData: 0,
                    SizeOfUninitializedData: 0,
                    AddressOfEntryPoint: 0,
                    BaseOfCode: 0,
                    ImageBase: 0,
                    SectionAlignment: 0,
                    FileAlignment: 0,
                    MajorOperatingSystemVersion: 0,
                    MinorOperatingSystemVersion: 0,
                    MajorImageVersion: 0,
                    MinorImageVersion: 0,
                    MajorSubsystemVersion: 0,
                    MinorSubsystemVersion: 0,
                    Win32VersionValue: 0,
                    SizeOfImage: 0,
                    SizeOfHeaders: 0,
                    CheckSum: 0,
                    Subsystem: 0,
                    DllCharacteristics: 0,
                    SizeOfStackReserve: 0,
                    SizeOfStackCommit: 0,
                    SizeOfHeapReserve: 0,
                    SizeOfHeapCommit: 0,
                    LoaderFlags: 0,
                    NumberOfRvaAndSizes: 0,
                    DataDirectory: [winapi::um::winnt::IMAGE_DATA_DIRECTORY {
                        VirtualAddress: 0,
                        Size: 0,
                    }; 16],
                },
            },
        }
    }

    pub fn parse_dos_stub(&mut self) {
        let szbuf = self.dos_header.e_lfanew as usize - std::mem::size_of::<IMAGE_DOS_HEADER>();
        let mut buffer = vec![0; szbuf];
        self.cursor.read_exact(&mut buffer).unwrap();
        self.dos_stub = buffer;
    }

    pub fn print_dos_stub(&self) {
        println!("DOS Stub:");
        for i in 0..self.dos_stub.len() {
            print!("{:02X} ", self.dos_stub[i]);
            if i % 16 == 15 {
                println!();
            }
        }
        println!();
    }

    pub fn parse_dos_header(&mut self) -> Result<(), Box<dyn Error>> {
        self.dos_header.e_magic = self.cursor.read_u16::<LittleEndian>()?;

        if self.dos_header.e_magic != 0x5A4D {
            return Err("Invalid DOS header".into());
        }

        self.dos_header.e_cblp = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_cp = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_crlc = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_cparhdr = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_minalloc = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_maxalloc = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_ss = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_sp = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_csum = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_ip = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_cs = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_lfarlc = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_ovno = self.cursor.read_u16::<LittleEndian>()?;
        for i in 0..4 {
            self.dos_header.e_res[i] = self.cursor.read_u16::<LittleEndian>()?;
        }
        self.dos_header.e_oemid = self.cursor.read_u16::<LittleEndian>()?;
        self.dos_header.e_oeminfo = self.cursor.read_u16::<LittleEndian>()?;
        for i in 0..10 {
            self.dos_header.e_res2[i] = self.cursor.read_u16::<LittleEndian>()?;
        }
        self.dos_header.e_lfanew = self.cursor.read_i32::<LittleEndian>()?;

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
