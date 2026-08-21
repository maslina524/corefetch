use alloc::{
    vec::Vec,
    vec
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

pub struct Image {
    w: usize,
    h: usize,
    data: Vec<Vec<Rgba>>
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h, data: vec![vec![Rgba::default(); h]; w] }
    }

    pub const fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn set_pixel(&mut self, p: Point, v: Rgba) -> Result<(), ()> {
        if p.x < self.w && p.y < self.h {
            self.data[p.x][p.y] = v;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Rgba> {
        self.data.get(x).and_then(|d| d.get(y))
    }
}