use std::io::{Seek, SeekFrom, Read};
use nom::bytes::streaming::take_till;
use nom_bufreader::Parse;
use nom_bufreader::bufreader::BufReader;
use bytes::BufMut;

use crate::entry::*;

pub(crate) fn dump_crc<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<CRC32>>{
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

pub(crate) fn dump_sha1<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<SHA1>>{
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

pub(crate) fn dump_name_addr<T: Seek + Read>(read: &mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<u32>>{
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

pub(crate) fn dump_names<'a, T: Seek + Read >(read: &'a mut BufReader<T>, off: u32, stop: usize) -> anyhow::Result<Vec<String>>{
    
    fn parse_name(i: &[u8]) -> nom::IResult<&[u8], String, ()>{
        let (i, res) = take_till(|c| c == 0)(i)?;
        Ok((i, String::from_utf8_lossy(res).to_string()))
    }

    let addrs = dump_name_addr(read, off, stop)?;
    let mut res = Vec::new();

    for addr in addrs {
        read.seek(SeekFrom::Start((addr - 0x400c00).into()))?;
        let name = read.parse(parse_name).map_err(|_e| anyhow::Error::msg("nom error"))?;
        res.push(name);
    }
    Ok(res)
}
