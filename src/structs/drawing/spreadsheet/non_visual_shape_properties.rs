// xdr:nvSpPr
use super::NonVisualDrawingProperties;
use crate::reader::driver::*;
use crate::writer::driver::*;
use crate::xml_read_loop_result;
use crate::XlsxError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct NonVisualShapeProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
}

impl NonVisualShapeProperties {
    #[inline]
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    #[inline]
    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    #[inline]
    pub fn set_non_visual_drawing_properties(&mut self, value: NonVisualDrawingProperties) {
        self.non_visual_drawing_properties = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) -> Result<(), XlsxError> {
        xml_read_loop_result!(
            reader,
            Event::Empty(ref e) => {
                if e.name().local_name().into_inner() == b"cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, true)?;
                }
            },
            Event::Start(ref e) => {
                if e.name().local_name().into_inner() == b"cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, false)?;
                }
            },
            Event::End(ref e) => {
                if e.name().local_name().into_inner() == b"nvSpPr" {
                    return Ok(());
                }
            },
            Event::Eof => return Err(XlsxError::XmlParse(
                "Could not find xdr:nvSpPr end element".into()
            ))
        )
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, ole_id: &usize) {
        // xdr:nvSpPr
        write_start_tag(writer, "xdr:nvSpPr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, ole_id);

        // xdr:cNvSpPr
        write_start_tag(writer, "xdr:cNvSpPr", vec![], true);

        write_end_tag(writer, "xdr:nvSpPr");
    }
}
