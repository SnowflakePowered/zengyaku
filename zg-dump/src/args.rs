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
    /// Do not output results.
    None,
    /// Output tab-separated values.
    Tsv,
    /// Output Logiqx XML.
    Xml,
}

/// zengyaku-dump: GoodTools database dumper
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub(crate) struct Args {
    /// The path to the executable to dump.
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

    /// The total number of known ROMs.
    #[clap(short, long, parse(try_from_str))]
    pub(crate) known_num: usize,

    /// The format to output results.
    #[clap(short, long, arg_enum, default_value_t = OutputFormat::None)]
    pub(crate) format: OutputFormat,

    /// The path to write output; if omitted, outputs to stdout.
    #[clap(short, long)]
    pub(crate) output: Option<PathBuf>,

    /// The extension to use when saving an Logiqx XML file; if omitted, emits no file extensions in the resulting `rom` entries.
    #[clap(short, long, default_value = "")]
    pub(crate) extension: String,
}