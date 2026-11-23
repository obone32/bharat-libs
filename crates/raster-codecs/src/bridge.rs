//! Compatibility between this crate's small-format facade and the canonical image codec model.

use crate::{ChannelLayout, CodecError, DecodeWarning, Image, Limits, SampleType};
use bharat_image_codecs::{DecodeOptions, Format, Metadata};

pub(crate) fn format(format: Format, bytes: &[u8], limits: &Limits) -> Result<Image, CodecError> {
    let shared_limits = bharat_image_codecs::Limits::new(limits.max_width, limits.max_height, limits.max_pixels, limits.max_alloc);
    let options = DecodeOptions { limits: shared_limits, ..DecodeOptions::default() };
    let image = bharat_image_codecs::decode_as_with(format, bytes, &options).map_err(from_shared_error)?;
    from_shared_image(image)
}

pub(crate) fn shared_image(image: &Image) -> Result<bharat_image_codecs::Image, CodecError> {
    let layout = match image.layout() {
        ChannelLayout::Gray => bharat_image_codecs::ChannelLayout::Gray,
        ChannelLayout::GrayA => bharat_image_codecs::ChannelLayout::GrayA,
        ChannelLayout::Rgb => bharat_image_codecs::ChannelLayout::Rgb,
        ChannelLayout::Rgba => bharat_image_codecs::ChannelLayout::Rgba,
        ChannelLayout::Cmyk => bharat_image_codecs::ChannelLayout::Cmyk,
        ChannelLayout::CmykA => bharat_image_codecs::ChannelLayout::CmykA,
    };
    let sample = match image.sample_type() {
        SampleType::U8 => bharat_image_codecs::SampleType::U8,
        SampleType::U16 => bharat_image_codecs::SampleType::U16,
        SampleType::F32 => bharat_image_codecs::SampleType::F32,
    };
    let meta = Metadata { dpi: image.dpi, text: image.text.clone(), ..Metadata::default() };
    bharat_image_codecs::Image::from_raw(image.width(), image.height(), layout, sample, image.data().to_vec())
        .map(|image| image.with_meta(meta))
        .map_err(from_shared_error)
}

fn from_shared_image(image: bharat_image_codecs::Image) -> Result<Image, CodecError> {
    let layout = match image.layout() {
        bharat_image_codecs::ChannelLayout::Gray => ChannelLayout::Gray,
        bharat_image_codecs::ChannelLayout::GrayA => ChannelLayout::GrayA,
        bharat_image_codecs::ChannelLayout::Rgb => ChannelLayout::Rgb,
        bharat_image_codecs::ChannelLayout::Rgba => ChannelLayout::Rgba,
        bharat_image_codecs::ChannelLayout::Cmyk => ChannelLayout::Cmyk,
        bharat_image_codecs::ChannelLayout::CmykA => ChannelLayout::CmykA,
        _ => return Err(CodecError::InvalidImage(format!("unsupported small-codec layout {:?}", image.layout()))),
    };
    let sample = match image.sample_type() {
        bharat_image_codecs::SampleType::U8 => SampleType::U8,
        bharat_image_codecs::SampleType::U16 => SampleType::U16,
        bharat_image_codecs::SampleType::F32 => SampleType::F32,
        _ => return Err(CodecError::InvalidImage(format!("unsupported small-codec sample type {:?}", image.sample_type()))),
    };
    let mut out = Image::from_raw(image.width(), image.height(), layout, sample, image.data().to_vec())?;
    out.dpi = image.meta.dpi;
    out.text = image.meta.text;
    out.warnings = image
        .warnings
        .into_iter()
        .filter_map(|warning| match warning {
            bharat_image_codecs::DecodeWarning::MorePages { total } | bharat_image_codecs::DecodeWarning::MoreParts { total } => {
                Some(DecodeWarning::MorePages { total })
            }
            _ => None,
        })
        .collect();
    Ok(out)
}

pub(crate) fn from_shared_error(error: bharat_image_codecs::CodecError) -> CodecError {
    match error {
        bharat_image_codecs::CodecError::UnknownFormat => CodecError::unsupported("image", "unrecognized image format"),
        bharat_image_codecs::CodecError::Unsupported { format, reason } => CodecError::unsupported(format.name(), reason),
        bharat_image_codecs::CodecError::Malformed { format, message } => CodecError::malformed(format.name(), message),
        bharat_image_codecs::CodecError::LimitExceeded(message) => CodecError::LimitExceeded(message),
        bharat_image_codecs::CodecError::Encode { format, message } => CodecError::Encode { format: format.name(), message },
        bharat_image_codecs::CodecError::InvalidImage(message) => CodecError::InvalidImage(message),
    }
}

pub(crate) fn encode(image: &Image, format: Format, options: &bharat_image_codecs::EncodeOptions) -> Result<Vec<u8>, CodecError> {
    let image = shared_image(image)?;
    bharat_image_codecs::encode(&image, format, options).map_err(from_shared_error)
}
