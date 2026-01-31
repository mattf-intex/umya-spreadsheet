// a:lnRef
use super::SchemeColor;
use crate::reader::driver::*;
use crate::writer::driver::*;
use crate::xml_read_loop_result;
use crate::XlsxError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct StyleMatrixReferenceType {
    index: Box<str>,
    scheme_color: Option<Box<SchemeColor>>,
}

impl StyleMatrixReferenceType {
    #[inline]
    pub fn get_index(&self) -> &str {
        &self.index
    }

    #[inline]
    pub fn set_index<S: Into<String>>(&mut self, value: S) {
        self.index = value.into().into_boxed_str();
    }

    #[inline]
    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_deref()
    }

    #[inline]
    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) -> Result<(), XlsxError> {
        self.set_index(get_attribute(e, b"idx").unwrap());

        if empty_flag {
            return Ok(());
        }

        xml_read_loop_result!(
            reader,
            Event::Start(ref e) => {
                if e.name().local_name().into_inner() == b"schemeClr" {
                    let mut scheme_color = SchemeColor::default();
                    scheme_color.set_attributes(reader, e, false)?;
                    self.set_scheme_color(scheme_color);
                }
            },
            Event::Empty(ref e) => {
                if e.name().local_name().into_inner() == b"schemeClr" {
                    let mut scheme_color = SchemeColor::default();
                    scheme_color.set_attributes(reader, e, true)?;
                    self.set_scheme_color(scheme_color);
                }
            },
            Event::End(ref e) => {
                match e.name().local_name().into_inner() {
                    b"lnRef" => {
                        return Ok(());
                    }
                    b"fillRef" => {
                        return Ok(());
                    }
                    b"effectRef" => {
                        return Ok(());
                    }
                    b"fontRef" => {
                        return Ok(());
                    }
                    _ => (),
                }
            },
            Event::Eof => return Err(XlsxError::XmlParse(
                "Could not find a:lnRef end element".into()
            ))
        )
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        if let Some(color) = &self.scheme_color {
            write_start_tag(writer, tag_name, vec![("idx", &self.index)], false);
            // a:schemeClr
            color.write_to(writer);
            write_end_tag(writer, tag_name);
        } else {
            write_start_tag(writer, tag_name, vec![("idx", &self.index)], true);
        }
    }
}
