//! PCX compatibility facade; implementation and header probing live in bharat-image-codecs.
use crate::{CodecError, Image, Limits, bridge};
use bharat_image_codecs::Format;

pub fn looks_like_pcx(bytes: &[u8]) -> bool {
    bharat_image_codecs::detect(bytes) == Some(Format::Pcx)
}
pub fn dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    bharat_image_codecs::pcx_dimensions(bytes)
}
pub fn decode(bytes: &[u8], limits: &Limits) -> Result<Image, CodecError> {
    bridge::format(Format::Pcx, bytes, limits)
}
