// a:noFill
use crate::reader::driver::*;
use crate::xml_read_loop_result;
use crate::XlsxError;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct NoFill {}
impl NoFill {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _: &BytesStart,
        empty_flag: bool,
    ) -> Result<(), XlsxError> {
        if empty_flag {
            return Ok(());
        }

        xml_read_loop_result!(
            reader,
            Event::End(ref e) => {
                if e.name().local_name().into_inner() == b"noFill" {
                    return Ok(());
                }
            },
            Event::Eof => return Err(XlsxError::XmlParse(
                "Could not find a:noFill end element".into()
            ))
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:noFill
        write_start_tag(writer, "a:noFill", vec![], true);
    }
}
