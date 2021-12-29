use std::io::{Cursor, Write};

use quick_xml::{Writer, events::{BytesStart, Event, BytesEnd, BytesText, BytesDecl}};
use anyhow::Result;
use crate::entry::Entry;

fn write_text_value<T: Write>(writer: &mut Writer<T>, key: &str, value: &str) -> Result<()> {
    writer.write_event(Event::Start(BytesStart::borrowed_name(key.as_bytes())))?; 
    {    
        writer.write_event(Event::Text(BytesText::from_escaped_str(value)))?;
    }
    writer.write_event(Event::End(BytesEnd::borrowed(key.as_bytes())))?;
    Ok(())
}

fn write_xml_header<T: Write>(writer: &mut Writer<T>, name: &str) -> Result<()> {
    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
    writer.write_event(Event::DocType(BytesText::from_escaped_str(r#" datafile PUBLIC "-//Logiqx//DTD ROM Management Datafile//EN" "http://www.logiqx.com/Dats/datafile.dtd""#)))?;    
    writer.write_event(Event::Start(BytesStart::borrowed_name(b"datafile")))?;
    writer.write_event(Event::Start(BytesStart::borrowed_name(b"header")))?;

    write_text_value(writer, "name", name)?;
    write_text_value(writer, "description", name)?;
    writer.write_event(Event::End(BytesEnd::borrowed(b"header")))?;
    Ok(())
}

pub(crate) fn write_xml(entries: &[Entry], ext: &str, src: &str) -> Result<String> {

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
    write_xml_header(&mut writer, src)?;

    for e in entries {
        let mut game = BytesStart::owned(b"game".to_vec(), "game".len());
        game.push_attribute((b"name" as &[u8], e.name.as_bytes()));
        writer.write_event(Event::Start(game))?;
        {
            write_text_value(&mut writer, "description", &e.name)?;            
            let rom_name = if ext != "" {
                format!("{}.{}", e.name.as_str(), ext)
            } else {
                e.name.clone()
            };

            let mut rom = BytesStart::owned(b"rom".to_vec(), "rom".len());
            let crc =format!("{:08x}", e.crc.0);
            rom.push_attribute((b"name" as &[u8], rom_name.as_bytes()));
            rom.push_attribute(("crc", crc.as_str()));
            rom.push_attribute(("sha1", hex::encode(e.sha1.0).as_str()));

            writer.write_event(Event::Empty(rom))?; 
        }
        writer.write_event(Event::End(BytesEnd::borrowed(b"game")))?;

    }
    writer.write_event(Event::End(BytesEnd::borrowed(b"datafile")))?;

    Ok(String::from_utf8(writer.into_inner().into_inner())?)
}

