// xdr:nvGraphicFramePr
use super::NonVisualDrawingProperties;
use super::NonVisualGraphicFrameDrawingProperties;
use crate::reader::driver::*;
use crate::writer::driver::*;
use crate::xml_read_loop_result;
use crate::XlsxError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct NonVisualGraphicFrameProperties {
    non_visual_drawing_properties: NonVisualDrawingProperties,
    non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
}

impl NonVisualGraphicFrameProperties {
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
    ) -> &mut NonVisualGraphicFrameProperties {
        self.non_visual_drawing_properties = value;
        self
    }

    #[inline]
    pub fn get_non_visual_graphic_frame_drawing_properties(
        &self,
    ) -> &NonVisualGraphicFrameDrawingProperties {
        &self.non_visual_graphic_frame_drawing_properties
    }

    #[inline]
    pub fn get_non_visual_graphic_frame_drawing_properties_mut(
        &mut self,
    ) -> &mut NonVisualGraphicFrameDrawingProperties {
        &mut self.non_visual_graphic_frame_drawing_properties
    }

    #[inline]
    pub fn set_non_visual_graphic_frame_drawing_properties(
        &mut self,
        value: NonVisualGraphicFrameDrawingProperties,
    ) -> &mut NonVisualGraphicFrameProperties {
        self.non_visual_graphic_frame_drawing_properties = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) -> Result<(), XlsxError> {
        xml_read_loop_result!(
            reader,
            Event::Empty(ref e) => {
                match e.name().local_name().into_inner() {
                    b"cNvPr" => {
                        self.non_visual_drawing_properties
                            .set_attributes(reader, e, true)?;
                    },
                    b"cNvGraphicFramePr" => {
                        self.non_visual_graphic_frame_drawing_properties
                            .set_attributes(reader, e);
                    },
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                if e.name().local_name().into_inner() == b"cNvPr" {
                    self.non_visual_drawing_properties
                        .set_attributes(reader, e, false)?;
                }
            },
            Event::End(ref e) => {
                if e.name().local_name().into_inner() == b"nvGraphicFramePr" {
                    return Ok(())
                }
            },
            Event::Eof => return Err(XlsxError::XmlParse(
                "Could not find xdr:nvGraphicFramePr end element".into()
            ))
        )
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:nvGraphicFramePr
        write_start_tag(writer, "xdr:nvGraphicFramePr", vec![], false);

        // xdr:cNvPr
        self.non_visual_drawing_properties.write_to(writer, &0);

        // xdr:cNvGraphicFramePr
        self.non_visual_graphic_frame_drawing_properties
            .write_to(writer);

        write_end_tag(writer, "xdr:nvGraphicFramePr");
    }
}
