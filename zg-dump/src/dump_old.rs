use std::io::{Seek, SeekFrom, Read};
use nom::IResult;
use nom::character::complete::{char, space1, newline, space0};
use nom::combinator::{opt, map_res, eof};
use nom::bytes::complete::{take_while_m_n, take_while};
use nom::bytes::streaming::take_till;
use nom::sequence::preceded;
use nom_bufreader::Parse;
use nom_bufreader::bufreader::BufReader;

use bytes::BufMut;
use crate::entry::*;

fn dump_entry_addr<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<u32>>{
    let mut results = Vec::new();
    results.reserve(stop);
    read.seek(SeekFrom::Start(off.into()))?;
    let mut buf = [0u8; 4];
    for _i in 0..stop {
        read.read_exact(&mut buf)?;
        results.push(u32::from_le_bytes(buf));
    }
    Ok(results)
}

fn dump_entries<'a, T: Seek + Read >(read: &'a mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<String>>{
    
    fn parse_cstring(i: &[u8]) -> nom::IResult<&[u8], String, ()>{
        let (i, res) = take_till(|c| c == 0)(i)?;
        Ok((i, String::from_utf8_lossy(res).to_string()))
    }

    let addrs = dump_entry_addr(read, off, stop)?;
    let mut res = Vec::new();

    for addr in addrs {
        read.seek(SeekFrom::Start((addr - 0x400c00).into()))?;
        let name = read.parse(parse_cstring).map_err(|e| anyhow::Error::msg(format!("Failed to parse name table ({:?})", e)))?;
        res.push(name);
    }
    Ok(res)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    fn is_hex_digit(c: char) -> bool {
        c.is_digit(16)
    }

    fn from_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(input, 16)
      }

    fn parse_crc32(input: &str) -> IResult<&str, CRC32> {
        let (input, crc) = map_res(take_while_m_n(8, 8, is_hex_digit), from_hex)(input)?;
        Ok((input, CRC32(crc)))
    }

    fn parse_sha1(input: &str) -> IResult<&str, SHA1> {
        let (input, b1) = map_res(take_while_m_n(8, 8, is_hex_digit), from_hex)(input)?;
        let (input, _) = space1(input)?;
        let (input, b2) = map_res(take_while_m_n(8, 8, is_hex_digit), from_hex)(input)?;
        let (input, _) = space1(input)?;
        let (input, b3) = map_res(take_while_m_n(8, 8, is_hex_digit), from_hex)(input)?;
        let (input, _) = space1(input)?;
        let (input, b4) = map_res(take_while_m_n(8, 8, is_hex_digit), from_hex)(input)?;
        let (input, _) = space1(input)?;
        let (input, b5) = map_res(take_while_m_n(8, 8, is_hex_digit), from_hex)(input)?;

        let mut buf = [0u8; 20];
        let mut dst = &mut buf[..];
        dst.put_u32(b1);
        dst.put_u32(b2);
        dst.put_u32(b3);
        dst.put_u32(b4);
        dst.put_u32(b5);
        Ok((input, SHA1(buf)))
    }

    let (input, crc) = parse_crc32(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = take_while(|c| c != '|' )(input)?;
    let (input, sha1) = opt(preceded(char('|'), parse_sha1))(input)?;

    // clear garbage and confirm end
    let (input, _) = space0(input)?;
    let (input, _) = opt(newline)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Entry { crc, name: String::from(name), sha1 }))
}

pub(crate) fn dump<'a, T: Seek + Read >(read: &'a mut BufReader<T>, entry_off: u32, known_num: usize) -> anyhow::Result<Vec<Entry>> {
    
    let entries= dump_entries(read, entry_off, known_num)?;
    let mut results = Vec::new();
    for e in entries.iter() {
        let (_s, e) = parse_entry(e).map_err(|e| anyhow::Error::msg(format!("Failed to parse entry table ({:?})", e)))?;
        results.push(e);
    }
    Ok(results)
}