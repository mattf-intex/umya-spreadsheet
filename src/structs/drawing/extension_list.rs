// a:extLst
use crate::reader::driver::*;
use crate::xml_read_loop_result;
use crate::XlsxError;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ExtensionList {}
impl ExtensionList {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) -> Result<(), XlsxError> {
        xml_read_loop_result!(
            reader,
            Event::End(ref e) => {
                if e.name().local_name().into_inner() == b"extLst" {
                    return Ok(())
                }
            },
            Event::Eof => return Err(XlsxError::XmlParse(
                "Could not find a:extLst end element".into()
            ))
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {}
}
