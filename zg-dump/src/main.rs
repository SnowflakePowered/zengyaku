use std::borrow::Cow;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::fs::File;
use clap::StructOpt;
use nom_bufreader::bufreader::BufReader;

use itertools::multizip;

mod args;
mod dump;
mod entry;
mod xml;

use args::OutputFormat;
use entry::*;

use crate::xml::write_xml;
fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    println!("Analyzing {:?} crc={} sha1={} name={} for {} entries", args.exe, args.crc_off, args.sha1_off, args.name_off, args.known_num);
    let file = File::open(&args.exe)?;
    let mut read = BufReader::new(file);

    let crc32 = dump::dump_crc(&mut read, args.crc_off, args.known_num)?;
    let sha1 = dump::dump_sha1(&mut read, args.sha1_off, args.known_num)?;
    let names = dump::dump_names(&mut read, args.name_off, args.known_num)?;

    if crc32.len() != args.known_num || sha1.len() != args.known_num || names.len() != args.known_num {
        return Err(anyhow::Error::msg("Mismatched number of items"));
    }
    
    let entries = collect(crc32, sha1, names);
    let mut output = String::new();
    match args.format {
        OutputFormat::None => {},
        OutputFormat::Tsv => {
            for entry in entries.iter() {
                writeln!(&mut output, "{}\t{:08x}\t{}", entry.name, entry.crc.0, hex::encode(entry.sha1.0))?;
            }
        }
        OutputFormat::Xml => {
            output = write_xml(&entries, &args.extension, args.exe.as_path().file_stem().map(|f| f.to_string_lossy())
                .unwrap_or(Cow::Borrowed("")).as_ref())?;
        }
    }

    if let Some(out_path) = args.output {
        let mut f = File::create(out_path)?;
        writeln!(&mut f, "{}", output)?;
    } else {
        println!("{}", output);
    }
    Ok(())
}

fn collect(crc: Vec<CRC32>, sha1: Vec<SHA1>, names: Vec<String>) -> Vec<Entry> { 
    let mut res = Vec::new();

    for (crc, sha1, name) in multizip((crc, sha1, names)) {
        res.push(Entry { 
            crc, sha1, name
        })
    }
    res
}