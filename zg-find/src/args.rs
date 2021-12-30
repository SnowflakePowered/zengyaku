use std::{path::PathBuf, num::ParseIntError, ffi::OsStr};
use clap::{Parser, Subcommand, ArgGroup};
use bytes::BufMut;

fn validate_file_exists(s: &OsStr) -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from(s);
    if path.exists() && path.is_file() {
        return Ok(path)
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found or not a file."))
}


fn try_hex_to_crc(s: &str) -> Result<[u8; 4], ParseIntError> {
    let int = u32::from_str_radix(s, 16)?;
    Ok(int.to_le_bytes())
}

fn try_hex_to_sha(s: &str) -> Result<[u8; 20], anyhow::Error> {
    let mut buf = [0u8; 20];
    if s.len() != 40 {
        return Err(anyhow::Error::msg("SHA1 was not 40 characters long"))
    }
    let b1 = u32::from_str_radix(&s[0..8], 16)?;
    let b2 = u32::from_str_radix(&s[8..16], 16)?;
    let b3 = u32::from_str_radix(&s[16..24], 16)?;
    let b4 = u32::from_str_radix(&s[24..32], 16)?;
    let b5 = u32::from_str_radix(&s[32..40], 16)?;

    let mut dst = &mut buf[..];
    dst.put_u32_le(b1);
    dst.put_u32_le(b2);
    dst.put_u32_le(b3);
    dst.put_u32_le(b4);
    dst.put_u32_le(b5);
    
    Ok(buf)
}



/// zengyaku-find: GoodTools Address Finder
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub(crate) struct Args {
    /// The path to the executable to search.
    #[clap(parse(try_from_os_str = validate_file_exists))]
    pub(crate) exe: PathBuf,    

    /// Output command-line arguments for zg-dump.
    #[clap(short='C', long)]
    pub(crate) print_args: bool,

    #[clap(subcommand)]
    pub(crate) command: GoodToolsVersion
}

#[derive(Debug, Subcommand)]
pub(crate) enum GoodToolsVersion {
    /// Find offsets for a version 3.2x database
    New {
        /// The CRC32 value to search for.
        #[clap(short, long, parse(try_from_str = try_hex_to_crc))]
        crc: [u8; 4],

        /// The SHA1 value to search for.
        #[clap(short, long, parse(try_from_str = try_hex_to_sha))]
        sha1: [u8; 20],

        /// The name string to search for.
        #[clap(short, long)]
        name: String,
    },
    /// Find offsets for a version 1 database
    #[clap(group(
        ArgGroup::new("v1")
            .required(true)
            .args(&["crc", "name"]),
    ))]
    Old {
        /// The CRC32 value to search for.
        #[clap(short, long, parse(try_from_str = try_hex_to_crc))]
        crc: Option<[u8; 4]>,

        /// The name string to search for.
        #[clap(short, long)]
        name: Option<String>,
    }
}