use core::error::Error;

use alloc::vec::Vec;

use crate::image::Image;

const PNG_SIG: &[u8] = b"\x89PNG\x0D\x0A\x1A\x0A";

#[derive(Debug)]
pub enum PngError {
    InvalidSignature,
    IncorrectDataLen
}

impl core::fmt::Display for PngError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSignature => write!(f, "PngError: InvalidSignature"),
            Self::IncorrectDataLen => write!(f, "PngError: IncorrectDataLen"),
        }
    }
}

impl Error for PngError {}

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

pub fn decode(bytes: &[u8]) -> Result<(), PngError> {
    let mut iter = bytes.iter().copied();
    if safe_take(&mut iter, PNG_SIG.len())? != PNG_SIG {
        return Err(PngError::InvalidSignature);
    }

    // IHDR


    Ok(())
}