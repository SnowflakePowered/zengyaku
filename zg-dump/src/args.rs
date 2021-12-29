use std::{path::PathBuf, num::ParseIntError, ffi::OsStr};
use clap::{Parser, ArgEnum};

fn try_hex_to_u32(s: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(s, 16)
}

fn validate_file_exists(s: &OsStr) -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from(s);
    if path.exists() && path.is_file() {
        return Ok(path)
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found or not a file."))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub(crate) enum OutputFormat {
    None,
    Tsv,
    Xml,
}

/// GoodTools Database Dumper
/// 
/// Must be ran on an unpacked executable
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub(crate) struct Args {
    /// The name of the executable to dump
    #[clap(parse(try_from_os_str = validate_file_exists))]
    pub(crate) exe: PathBuf,

    /// The offset of the CRC32 table.
    #[clap(short, long, parse(try_from_str = try_hex_to_u32))]
    pub(crate) crc_off: u32,

    /// The offset of the SHA1 table.
    #[clap(short, long, parse(try_from_str = try_hex_to_u32))]
    pub(crate) sha1_off: u32,

    /// The offset of the name table.
    #[clap(short, long, parse(try_from_str = try_hex_to_u32))]
    pub(crate) name_off: u32,

    /// Total number of known ROMs
    #[clap(short, long, parse(try_from_str))]
    pub(crate) known_num: usize,

    /// Output format
    #[clap(arg_enum, default_value_t = OutputFormat::None)]
    pub(crate) format: OutputFormat,

    /// The name of the executable to dump
    #[clap(short, long)]
    pub(crate) output: Option<PathBuf>,

    /// The extension to use when saving an Logiqx XML file.
    #[clap(short, long, default_value = "")]
    pub(crate) extension: String,
}