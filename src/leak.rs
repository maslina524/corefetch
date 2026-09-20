use core::fmt::Write;

#[derive(Clone, Copy)]
pub struct ConcatStr<const N: usize> {
    parts: [&'static str; N]
}

impl<const N: usize> ConcatStr<N> {
    pub const fn new(parts: [&'static str; N]) -> Self {
        Self { parts }
    }

    pub fn len(&self) -> usize {
        self.parts.iter().map(|s| s.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.iter().all(|s| s.is_empty())
    }

    pub fn chunks(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.parts.iter().copied()
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.parts.iter().flat_map(|s| s.bytes())
    }
}

impl<const N: usize> core::fmt::Display for ConcatStr<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for p in self.parts {
            f.write_str(p)?;
        }
        Ok(())
    }
}

impl<const N: usize> core::fmt::Debug for ConcatStr<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_char('"')?;
        for p in self.parts {
            for c in p.chars() {
                c.escape_debug().fmt(f)?;
            }
        }
        f.write_char('"')?;
        Ok(())
    }
}

impl<const N: usize> Default for ConcatStr<N> {
    fn default() -> Self {
        Self::new([""; N])
    }
}