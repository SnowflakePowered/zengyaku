use std::io::{BufReader, Read};
use std::fs::File;
use clap::StructOpt;
use memchr::memmem;

mod args;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    println!("Finding offsets for {:?}", args.exe);
    let file = File::open(&args.exe)?;
    let mut read = BufReader::new(file);
    let mut binary = Vec::new();
    read.read_to_end(&mut binary)?;

    for idx in memmem::find_iter(&binary, &args.crc) {
        println!("CRC: {:08x}", idx);
    }
    for idx in memmem::find_iter(&binary, &args.sha1) {
        println!("SHA1: {:08x}", idx);
    }
    for idx in memmem::find_iter(&binary, &args.name.as_bytes()) {
        let byte_addr = ((idx + 0x400c00) as u32).to_le_bytes();
        for idx in memmem::find_iter(&binary, &byte_addr) {
            println!("Name: {:08x}", idx);
        }
    }
    Ok(())
}
