use core::error::Error;

use alloc::vec::Vec;

use crate::image::Image;

const PNG_SIG: &[u8] = b"\x89PNG\x0D\x0A\x1A\x0A";
const IHDR_NAME: &[u8] = b"IHDR";

#[derive(Debug)]
pub enum PngError {
    InvalidSignature,
    IncorrectDataLen,
    FirstChunkIsNotIHDR,
    IncorrectIHDRLen,
    IncorrectIHDRName,
    TypeIsNotDeflate
}

impl core::fmt::Display for PngError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSignature    => write!(f, "Png: Invalid signature"),
            Self::IncorrectDataLen    => write!(f, "Png: Incorrect data len"),
            Self::FirstChunkIsNotIHDR => write!(f, "Png: First chunk is not IHDR"),
            Self::IncorrectIHDRLen    => write!(f, "Png: Incorrect IHDR len"),
            Self::IncorrectIHDRName   => write!(f, "Png: Incorrect IHDR name"),
            Self::TypeIsNotDeflate    => write!(f, "Png: Type is not Deflate"),
        }
    }
}

impl Error for PngError {}

pub struct Ihdr {
    pub width: u32,
    pub height: u32,
    pub depth: u8,
    pub typ: u8,
    pub method: u8
}

impl Ihdr {
    pub fn new<I>(iter: &mut I) -> Result<Self, PngError>
    where
        I: Iterator<Item = u8>,
    {
        let len = u32_from_bytes(&safe_take(iter, 4)?);
        if len != 13 { return Err(PngError::IncorrectIHDRLen); }

        let name = safe_take(iter, 4)?;
        if name != IHDR_NAME { return Err(PngError::IncorrectIHDRName); }

        let width = u32_from_bytes(&safe_take(iter, 4)?);
        let height = u32_from_bytes(&safe_take(iter, 4)?);
        let depth = iter.next().ok_or(PngError::IncorrectDataLen)?;
        let typ = iter.next().ok_or(PngError::IncorrectDataLen)?;

        let method = iter.next().ok_or(PngError::IncorrectDataLen)?;
        if method != 0 { return Err(PngError::TypeIsNotDeflate); }

        let _ = safe_take(iter, 2)?;

        Ok(Self {
            width, height, depth, typ, method,
        })
    }
}

fn safe_take<I>(iter: &mut I, n: usize) -> Result<Vec<I::Item>, PngError>
where
    I: Iterator
{   
    let taken: Vec<I::Item> = iter.by_ref().take(n).collect();
    
    if taken.len() == n {
        Ok(taken)
    } else {
        Err(PngError::IncorrectDataLen)
    }
}

fn u32_from_bytes(vec: &[u8]) -> u32 {
    (vec[0] as u32)
        | (vec[1] as u32) << 8
        | (vec[2] as u32) << 16
        | (vec[3] as u32) << 24
}

pub fn decode(bytes: &[u8]) -> Result<(), PngError> {
    let mut iter = bytes.iter().copied();
    if safe_take(&mut iter, PNG_SIG.len())? != PNG_SIG {
        return Err(PngError::InvalidSignature);
    }

    // IHDR
    let ihdr = Ihdr::new(&mut iter)?;

    Ok(())
}