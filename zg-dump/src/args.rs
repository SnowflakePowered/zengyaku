use std::{path::PathBuf, num::ParseIntError, ffi::OsStr};
use clap::{Parser, ArgEnum, ArgGroup};

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
    /// Output tab-separated values.
    Tsv,
    /// Output Logiqx XML.
    Xml,
}

/// zengyaku-dump: GoodTools database dumper
#[derive(Parser, Debug)]
#[clap(about, version, author)]
#[clap(group(
    ArgGroup::new("new")
        .args(&["crc-off", "sha1-off", "name-off"])
        .multiple(true)
        .conflicts_with("entry-off")
        .requires_all(&["crc-off", "sha1-off", "name-off"])
))]
pub(crate) struct Args {
    /// The path to the executable to dump.
    #[clap(parse(try_from_os_str = validate_file_exists))]
    pub(crate) exe: PathBuf,

    #[clap(display_order=2)]
    /// The offset of the entry table for old-style databases.
    #[clap(short, long, required_unless_present_all(&["crc-off", "sha1-off", "name-off"]), parse(try_from_str = try_hex_to_u32))]
    pub(crate) entry_off: Option<u32>,

    #[clap(display_order=1)]
    /// The offset of the CRC32 table for new-style databases.
    #[clap(short, long, parse(try_from_str = try_hex_to_u32))]
    pub(crate) crc_off: Option<u32>,

    #[clap(display_order=1)]
    /// The offset of the SHA1 table for new-style databases.
    #[clap(short, long, parse(try_from_str = try_hex_to_u32))]
    pub(crate) sha1_off: Option<u32>,

    #[clap(display_order=1)]
    /// The offset of the name table for new-style databases.
    #[clap(short, long,  parse(try_from_str = try_hex_to_u32))]
    pub(crate) name_off: Option<u32>,

    #[clap(display_order=0)]
    /// The total number of known ROMs.
    #[clap(short, long, parse(try_from_str))]
    pub(crate) known_num: usize,

    /// The format to output results.
    #[clap(display_order=3)]
    #[clap(short, long, arg_enum)]
    pub(crate) format: Option<OutputFormat>,

    /// The path to write output; if omitted, outputs to stdout.
    #[clap(short, long, requires("format"))]
    #[clap(display_order=3)]
    pub(crate) output: Option<PathBuf>,

    /// The extension to use when saving an Logiqx XML file; if omitted, emits no file extensions in the resulting `rom` entries.
    #[clap(short='x', long, default_value = "")]
    #[clap(display_order=4)]
    pub(crate) extension: String,

    /// Base address to use when doing pointer calculations.
    #[clap(long, default_value = "400c00", display_order=1000, parse(try_from_str = try_hex_to_u32))]
    pub(crate) base_address: u32,
}