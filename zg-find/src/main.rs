use std::io::{BufReader, Read};
use std::fs::File;
use std::fmt::Write;
use args::GoodToolsVersion;
use clap::StructOpt;
use memchr::memmem;

mod args;

const STAT_STR: &'static str = "\nStats: %d entries";
struct SearchResult {
    crc_off: Option<usize>,
    sha1_off: Option<usize>,
    name_off: Option<usize>,
    known_num: Option<i32>
}

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    if !args.print_args {
        println!("Finding offsets for {:?}", args.exe);
    }

    let file = File::open(&args.exe)?;
    let mut read = BufReader::new(file);
    let mut binary = Vec::new();
    read.read_to_end(&mut binary)?;

    match args.command {
        GoodToolsVersion::New { crc ,sha1 , name } => {
            let (res, out) = find_new(&binary, &crc, &sha1, &name)?;
            if args.print_args {
                println!(r#"-c {:08x} -s {:08x} -n {:08x} -k {} "{}""#, res.crc_off.unwrap_or(0), res.sha1_off.unwrap_or(0), res.name_off.unwrap_or(0), res.known_num.unwrap_or(0), &args.exe.as_os_str().to_string_lossy())
            } else {
                print!("{}", out);
            }
            
            if res.crc_off.is_none() || res.sha1_off.is_none() || res.name_off.is_none() || res.known_num.is_none() {
                Err(anyhow::Error::msg("Unable to find all necessary offsets"))
            } else {
                Ok(())
            }
        },
        GoodToolsVersion::Old { crc, name} => {
            let (res, out) = find_old(&binary, crc, name)?;
            if args.print_args {
                println!(r#"-e {:08x} -k {} "{}""#, res.crc_off.unwrap_or(0), res.known_num.unwrap_or(0), &args.exe.as_os_str().to_string_lossy())
            } else {
                print!("{}", out);
            }
            Ok(())
        }
    }
}

fn find_old(binary: &[u8], crc: Option<[u8; 4]>, name: Option<String>) -> anyhow::Result<(SearchResult, String)> {
    let mut out = String::new();
    let mut crc_off = None;
    let mut known_num = None;
    if let Some(crc) = crc {
    let crc_str = format!("{:08x}", u32::from_le_bytes(crc));
        for idx in memmem::find_iter(&binary, &crc_str.as_bytes()) {
            let byte_addr = ((idx + 0x400c00) as u32).to_le_bytes();
            for idx in memmem::find_iter(&binary, &byte_addr) {
                writeln!(out, "Entry: {:08x}", idx)?;
                if crc_off.is_none() {
                    crc_off = Some(idx);
                }
            }
        }
    } else if let Some(name) = name.as_ref() {
        for idx in memmem::find_iter(&binary, name.as_bytes()) {
            let byte_addr = ((idx - 0x9 + 0x400c00) as u32).to_le_bytes();
            for idx in memmem::find_iter(&binary, &byte_addr) {
                writeln!(out, "Entry: {:08x}", idx)?;
                if crc_off.is_none() {
                    crc_off = Some(idx);
                }
            }
        }
    }

    
    for idx in memmem::find_iter(&binary, STAT_STR.as_bytes()) {
        let byte_addr = ((idx + 0x400c00) as u32).to_le_bytes();
        if let Some(idx) =  memmem::find_iter(&binary, &byte_addr).next() {
            // 2 PUSH (0x68) before this
            if let Some(push_idx) = memmem::rfind_iter(&binary[..idx], &[0x68]).nth(1) {
                let mut buf = [0u8; 4];
                buf.clone_from_slice(&binary[push_idx+1..=push_idx+4]);
                let num = i32::from_le_bytes(buf);
                writeln!(out, "Known: {}", num)?;
                if known_num.is_none() {
                    known_num = Some(num);
                }
            }
        }
     }
 
    Ok((SearchResult { crc_off, sha1_off: None, name_off: None, known_num }, out))
}

fn find_new(binary: &[u8], crc: &[u8; 4], sha1: &[u8; 20], name: &str) -> anyhow::Result<(SearchResult, String)> {
    let mut crc_off = None;
    let mut sha1_off = None;
    let mut name_off = None;
    let mut known_num = None;
    let mut out = String::new();
    for idx in memmem::find_iter(&binary, crc) {
        writeln!(out, "CRC: {:08x}", idx)?;
        if crc_off.is_none() {
            crc_off = Some(idx);
        }
    }
    for idx in memmem::find_iter(&binary, sha1) {
        writeln!(out, "SHA1: {:08x}", idx)?;
        if sha1_off.is_none() {
            sha1_off = Some(idx);
        }
    }
    for idx in memmem::find_iter(&binary, name.as_bytes()) {
        let byte_addr = ((idx + 0x400c00) as u32).to_le_bytes();
        for idx in memmem::find_iter(&binary, &byte_addr) {
            writeln!(out, "Name: {:08x}", idx)?;
            if name_off.is_none() {
                name_off = Some(idx);
            }
        }
    }

    for idx in memmem::find_iter(&binary, STAT_STR.as_bytes()) {
        let byte_addr = ((idx + 0x400c00) as u32).to_le_bytes();
        if let Some(idx) =  memmem::find_iter(&binary, &byte_addr).next() {
            // 2 PUSH (0x68) before this
            if let Some(push_idx) = memmem::rfind_iter(&binary[..idx], &[0x68]).nth(1) {
                let mut buf = [0u8; 4];
                buf.clone_from_slice(&binary[push_idx+1..=push_idx+4]);
                let num = i32::from_le_bytes(buf);
                writeln!(out, "Known: {}", num)?;
                if known_num.is_none() {
                    known_num = Some(num);
                }
            }
        }
     }

    Ok((SearchResult { crc_off, sha1_off, name_off, known_num }, out))
}