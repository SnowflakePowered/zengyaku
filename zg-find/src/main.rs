use std::io::{BufReader, Read};
use std::fs::File;
use std::fmt::Write;
use clap::StructOpt;
use memchr::memmem;

mod args;

const HAVE_LIST_FN_SIG: [u8; 5] = [0xff, 0x83, 0xc4, 0x20, 0x8d];

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    if !args.print_args {
        println!("Finding offsets for {:?}", args.exe);
    }

    let file = File::open(&args.exe)?;
    let mut read = BufReader::new(file);
    let mut binary = Vec::new();
    read.read_to_end(&mut binary)?;

    let mut crc_off = None;
    let mut sha1_off = None;
    let mut name_off = None;
    let mut known_num = None;
    let mut out = String::new();
    for idx in memmem::find_iter(&binary, &args.crc) {
        writeln!(out, "CRC: {:08x}", idx)?;
        if crc_off.is_none() {
            crc_off = Some(idx);
        }
    }
    for idx in memmem::find_iter(&binary, &args.sha1) {
        writeln!(out, "SHA1: {:08x}", idx)?;
        if sha1_off.is_none() {
            sha1_off = Some(idx);
        }
    }
    for idx in memmem::find_iter(&binary, &args.name.as_bytes()) {
        let byte_addr = ((idx + 0x400c00) as u32).to_le_bytes();
        for idx in memmem::find_iter(&binary, &byte_addr) {
            writeln!(out, "Name: {:08x}", idx)?;
            if name_off.is_none() {
                name_off = Some(idx);
            }
        }
    }

    for idx in memmem::find_iter(&binary, &HAVE_LIST_FN_SIG) {
       // 3 PUSH (0x68) before this
       if let Some(push_idx) = memmem::rfind_iter(&binary[..idx], &[0x68]).nth(2) {
            let mut buf = [0u8; 4];
            buf.clone_from_slice(&binary[push_idx+1..=push_idx+4]);
            let num = i32::from_le_bytes(buf);
            writeln!(out, "Known: {}", num)?;
            if known_num.is_none() {
                known_num = Some(num);
            }
       }
    }

    if args.print_args {
        println!(r#"-c {:08x} -s {:08x} -n {:08x} -k {} "{}""#, crc_off.unwrap_or(0), sha1_off.unwrap_or(0), name_off.unwrap_or(0), known_num.unwrap_or(0), &args.exe.as_os_str().to_string_lossy())
    } else {
        print!("{}", out);
    }

    if crc_off.is_none() || sha1_off.is_none() || name_off.is_none() || known_num.is_none() {
        Err(anyhow::Error::msg("Unable to find all necessary offsets"))
    } else {
        Ok(())
    }
}
