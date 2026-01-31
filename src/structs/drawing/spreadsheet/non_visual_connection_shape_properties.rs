// xdr:nvCxnSpPr
use super::NonVisualConnectorShapeDrawingProperties;
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
pub struct NonVisualConnectionShapeProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
}

impl NonVisualConnectionShapeProperties {
    #[inline]
    pub fn get_non_visual_drawing_properties(&self) -> &NonVisualDrawingProperties {
        &self.non_visual_drawing_properties
    }

    #[inline]
    pub fn get_non_visual_drawing_properties_mut(&mut self) -> &mut NonVisualDrawingProperties {
        &mut self.non_visual_drawing_properties
    }

    #[inline]
    pub fn set_non_visual_drawing_properties(
        &mut self,
        value: NonVisualDrawingProperties,
    ) -> &mut NonVisualConnectionShapeProperties {
        self.non_visual_drawing_properties = value;
        self
    }

    #[inline]
    pub fn get_non_visual_connector_shape_drawing_properties(
        &self,
    ) -> &NonVisualConnectorShapeDrawingProperties {
        &self.non_visual_connector_shape_drawing_properties
    }

    #[inline]
    pub fn get_non_visual_connector_shape_drawing_properties_mut(
        &mut self,
    ) -> &mut NonVisualConnectorShapeDrawingProperties {
        &mut self.non_visual_connector_shape_drawing_properties
    }

    #[inline]
    pub fn set_non_visual_connector_shape_drawing_properties(
        &mut self,
        value: NonVisualConnectorShapeDrawingProperties,
    ) -> &mut NonVisualConnectionShapeProperties {
        self.non_visual_connector_shape_drawing_properties = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) -> Result<(), XlsxError> {
        xml_read_loop_result!(
            reader,
            Event::Start(ref e) => {
                if e.name().local_name().into_inner() == b"cNvCxnSpPr" {
                    self.non_visual_connector_shape_drawing_properties
                        .set_attributes(reader, e)?;
                }
            },
            Event::Empty(ref e) => {
                if e.name().local_name().into_inner() == b"cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, true)?;
                }
            },
            Event::End(ref e) => {
                if e.name().local_name().into_inner() == b"nvCxnSpPr" {
                    return Ok(())
                }
            },
            Event::Eof => return Err(XlsxError::XmlParse(
                "Could not find xdr:nvCxnSpPr end element".into()
            ))
        )
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvCxnSpPr
        write_start_tag(writer, "xdr:nvCxnSpPr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvCxnSpPr
        self.non_visual_connector_shape_drawing_properties
            .write_to(writer);

        write_end_tag(writer, "xdr:nvCxnSpPr");
    }
}
