use alloc::{
    vec::Vec,
    vec
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Image {
    w: usize,
    h: usize,
    data: Vec<Rgba>
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, data: vec![Rgba::default(); w * h] }
    }

    pub const fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, v: Rgba) -> Result<(), ()> {
        if x < self.w && y < self.h {
            self.data[y * self.w + x] = v;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Rgba> {
        self.data.get(y * self.w + x)
    }
}