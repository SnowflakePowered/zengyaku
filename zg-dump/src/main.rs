use std::borrow::Cow;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::fs::File;
use clap::StructOpt;
use nom_bufreader::bufreader::BufReader;


mod args;
mod dump_new;
mod dump_old;
mod entry;
mod xml;

use args::OutputFormat;

use crate::xml::write_xml;
fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let file = File::open(&args.exe)?;
    let mut read = BufReader::new(file);
    let mut output = String::new();

    let entries = if args.entry_off.is_none() {
        if args.format.is_none() || args.output.is_some() {
            println!("Analyzing new-style database {:?} crc={} sha1={} name={} for {} entries", args.exe, args.crc_off.unwrap(), args.sha1_off.unwrap(), args.name_off.unwrap(), args.known_num);
        }
        dump_new::dump(&mut read, args.crc_off.unwrap(), args.sha1_off.unwrap(), args.name_off.unwrap(), args.known_num)?
    } else {
        if args.format.is_none() || args.output.is_some() {
            println!("Analyzing old-style database {:?} entry={} for {} entries", args.exe, args.entry_off.unwrap(), args.known_num);
        }

        dump_old::dump(&mut read,args.entry_off.unwrap(), args.known_num)?
    };

    match args.format {
        None => {},
        Some(OutputFormat::Tsv) => {
            for entry in entries.iter() {
                if let Some(sha1) = &entry.sha1 {
                    writeln!(&mut output, "{}\t{:08x}\t{}", entry.name, entry.crc.0, hex::encode(sha1.0))?;
                } else {
                    writeln!(&mut output, "{}\t{:08x}", entry.name, entry.crc.0)?;
                }
            }
        }
        Some(OutputFormat::Xml) => {
            output = write_xml(&entries, &args.extension, args.exe.as_path().file_stem().map(|f| f.to_string_lossy())
                .unwrap_or(Cow::Borrowed("")).as_ref())?;
        }
    }

    if let Some(out_path) = args.output {
        let mut f = File::create(out_path)?;
        writeln!(&mut f, "{}", output)?;
    } else {
        print!("{}", output);
    }
    Ok(())
}

