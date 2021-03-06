use std::io::{Seek, SeekFrom, Read};
use nom::bytes::streaming::take_till;
use nom_bufreader::Parse;
use nom_bufreader::bufreader::BufReader;
use bytes::BufMut;
use itertools::multizip;

use crate::entry::*;

fn dump_crc<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<CRC32>>{
    let mut results = Vec::new();
    results.reserve(stop);
    read.seek(SeekFrom::Start(off.into()))?;
    let mut buf = [0u8; 4];
    for _i in 0..stop {
        read.read_exact(&mut buf)?;
        let crc32 = CRC32(u32::from_le_bytes(buf));
        results.push(crc32);
    }
    Ok(results)
}

fn dump_sha1<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<SHA1>>{
    let mut results = Vec::new();
    results.reserve(stop);
    read.seek(SeekFrom::Start(off.into()))?;
    let mut int_buf = [0u8; 4];
    let mut buf = [0u8; 20];
    for _i in 0..stop {
        let mut dst = &mut buf[..];
        read.read_exact(&mut int_buf)?;
        dst.put_u32(u32::from_le_bytes(int_buf));

        read.read_exact(&mut int_buf)?;
        dst.put_u32(u32::from_le_bytes(int_buf));

        read.read_exact(&mut int_buf)?;
        dst.put_u32(u32::from_le_bytes(int_buf));

        read.read_exact(&mut int_buf)?;
        dst.put_u32(u32::from_le_bytes(int_buf));
        
        read.read_exact(&mut int_buf)?;
        dst.put_u32(u32::from_le_bytes(int_buf));
        let sha1 = SHA1(buf);
        results.push(sha1);
    }
    Ok(results)
}

fn dump_name_addr<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<u32>>{
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

fn dump_names<'a, T: Seek + Read >(read: &'a mut BufReader<T>, off: u32, stop: usize, base: u32) -> anyhow::Result<Vec<String>>{
    
    fn parse_name(i: &[u8]) -> nom::IResult<&[u8], String, ()>{
        let (i, res) = take_till(|c| c == 0)(i)?;
        Ok((i, String::from_utf8_lossy(res).to_string()))
    }

    let addrs = dump_name_addr(read, off, stop)?;
    let mut res = Vec::new();

    for addr in addrs {
        read.seek(SeekFrom::Start((addr - base).into()))?;
        let name = read.parse(parse_name).map_err(|e| anyhow::Error::msg(format!("Failed to parse name table ({:?})", e)))?;
        res.push(name);
    }
    Ok(res)
}

fn collect(crc: Vec<CRC32>, sha1: Vec<SHA1>, names: Vec<String>) -> Vec<Entry> { 
    let mut res = Vec::new();

    for (crc, sha1, name) in multizip((crc, sha1, names)) {
        res.push(Entry { 
            crc, sha1: Some(sha1), name
        })
    }
    res
}

pub(crate) fn dump<'a, T: Seek + Read >(read: &'a mut BufReader<T>, crc_off: u32, sha1_off: u32, name_off: u32, known_num: usize, base: u32) -> anyhow::Result<Vec<Entry>> {
    let crc32 = dump_crc(read, crc_off, known_num)?;
    let sha1 = dump_sha1(read, sha1_off,known_num)?;
    let names = dump_names(read, name_off, known_num, base)?;

    if [crc32.len(), sha1.len(), names.len()] != [known_num, known_num, known_num] {
        return Err(anyhow::Error::msg("Mismatched number of items"));
    }
    
    Ok(collect(crc32, sha1, names))
}