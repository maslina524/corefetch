use core::error::Error;

use alloc::vec::Vec;

use crate::{
    deflate::{self, DeflateError},
    image::{
        ColorType, ColorTypeError, Image, Rgba, RgbaConvertError
    }
};

const PNG_SIG: &[u8] = b"\x89PNG\x0D\x0A\x1A\x0A";
const IHDR_NAME: &[u8] = b"IHDR";

#[derive(Debug)]
pub enum PngError {
    InvalidSignature,
    IncorrectDataLen,
    FirstChunkIsNotIHDR,
    IncorrectIHDRLen,
    IncorrectIHDRName,
    UnsupportedCompression,
    UnsupportedFilter,
    UnsupportedInterlace,
    UnexpectedEof,
    DeflateError(DeflateError),
    ColorTypeError(ColorTypeError),
    RgbaConvertError(RgbaConvertError)
}

impl core::fmt::Display for PngError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSignature                       => write!(f, "Png: Invalid signature"),
            Self::IncorrectDataLen                       => write!(f, "Png: Incorrect data len"),
            Self::FirstChunkIsNotIHDR                    => write!(f, "Png: First chunk is not IHDR"),
            Self::IncorrectIHDRLen                       => write!(f, "Png: Incorrect IHDR len"),
            Self::IncorrectIHDRName                      => write!(f, "Png: Incorrect IHDR name"),
            Self::UnsupportedCompression                 => write!(f, "Png: Unsupported compression method (only Deflate)"),
            Self::UnsupportedFilter                      => write!(f, "Png: Unsupported filter method (only 0)"),
            Self::UnsupportedInterlace                   => write!(f, "Png: Unsupported interlace method (only 0)"),
            Self::UnexpectedEof                          => write!(f, "Png: Unexpected end of data"),
            Self::DeflateError(e)         => write!(f, "{e}"),
            Self::ColorTypeError(e)     => write!(f, "{e}"),
            Self::RgbaConvertError(e) => write!(f, "{e}"),
        }
    }
}

impl From<DeflateError> for PngError {
    fn from(value: DeflateError) -> Self {
        Self::DeflateError(value)
    }
}

impl From<ColorTypeError> for PngError {
    fn from(value: ColorTypeError) -> Self {
        Self::ColorTypeError(value)
    }
}

impl From<RgbaConvertError> for PngError {
    fn from(value: RgbaConvertError) -> Self {
        Self::RgbaConvertError(value)
    }
}

impl Error for PngError {}

struct Ihdr {
    pub width: usize,
    pub height: usize,
    pub depth: u8,
    pub color_type: ColorType,
    pub compression: u8,
    pub filter: u8,
    pub interlace: u8,
}

impl Ihdr {
    pub fn new<I>(iter: &mut I) -> Result<Self, PngError>
    where
        I: Iterator<Item = u8>,
    {
        let len = u32_from_bytes(&safe_take(iter, 4)?);
        if len != 13 {
            return Err(PngError::IncorrectIHDRLen);
        }

        let name = safe_take(iter, 4)?;
        if name != IHDR_NAME {
            return Err(PngError::IncorrectIHDRName);
        }

        let width = u32_from_bytes(&safe_take(iter, 4)?) as usize;
        let height = u32_from_bytes(&safe_take(iter, 4)?) as usize;
        let depth = iter.next().ok_or(PngError::UnexpectedEof)?;
        let color_type_u8 = iter.next().ok_or(PngError::UnexpectedEof)?;
        let color_type = ColorType::try_from(color_type_u8)?;
        let compression = iter.next().ok_or(PngError::UnexpectedEof)?;
        let filter = iter.next().ok_or(PngError::UnexpectedEof)?;
        let interlace = iter.next().ok_or(PngError::UnexpectedEof)?;

        if compression != 0 {
            return Err(PngError::UnsupportedCompression);
        }
        if filter != 0 {
            return Err(PngError::UnsupportedFilter);
        }
        if interlace != 0 {
            return Err(PngError::UnsupportedInterlace);
        }

        let _crc = safe_take(iter, 4)?;

        Ok(Self {
            width,
            height,
            depth,
            color_type,
            compression,
            filter,
            interlace,
        })
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub name: [u8; 4],
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn new<I>(iter: &mut I) -> Result<Self, PngError>
    where
        I: Iterator<Item = u8>,
    {
        let len = u32_from_bytes(&safe_take(iter, 4)?);
        let name_bytes = safe_take(iter, 4)?;
        let mut name = [0u8; 4];
        name.copy_from_slice(&name_bytes);

        let raw = safe_take(iter, len as usize)?;
        let _crc = safe_take(iter, 4)?; // пропуск CRC

        Ok(Self { name, data: raw })
    }
}

fn safe_take<I>(iter: &mut I, n: usize) -> Result<Vec<I::Item>, PngError>
where
    I: Iterator<Item = u8>,
{
    let taken: Vec<u8> = iter.by_ref().take(n).collect();
    if taken.len() == n {
        Ok(taken)
    } else {
        Err(PngError::IncorrectDataLen)
    }
}

fn u32_from_bytes(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

pub fn decode(bytes: &[u8]) -> Result<Image, PngError> {
    let mut iter = bytes.iter().copied();

    if safe_take(&mut iter, PNG_SIG.len())? != PNG_SIG {
        return Err(PngError::InvalidSignature);
    }
 
    let ihdr = Ihdr::new(&mut iter)?;

    let mut idat_data: Vec<u8> = Vec::new();
    loop {
        let chunk = Chunk::new(&mut iter)?;
        crate::println!("{}: {:?}", str::from_utf8(&chunk.name).unwrap(), chunk.data);
        match &chunk.name {
            b"IEND" => {
                break;
            },
            b"IDAT" => {
                idat_data.extend_from_slice(&chunk.data);
            },
            _ => {}
        }
    }

    let deflate_raw = &idat_data[2..idat_data.len() - 4];
    let decompressed = deflate::decode(deflate_raw)?;
    let mut iter = decompressed.iter().copied();
    
    let mut image = Image::new(ihdr.width, ihdr.height);
    for y in 0..ihdr.height {
        iter.next(); // Filter
        for x in 0..ihdr.width {
            // RGBA
            let pixel_raw = safe_take(&mut iter, 4)?;
            let pixel = Rgba(pixel_raw[0], pixel_raw[1], pixel_raw[2], pixel_raw[3]);
            image.set_pixel(x, y, pixel).unwrap();
        }
    }

    Ok(image)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::png;

    #[test]
    fn png_marker_test() {
        let data = fs::read("test/png/marker.png").unwrap();
        let image = png::decode(&data).unwrap();
        
        println!("{image:?}");
    }
}