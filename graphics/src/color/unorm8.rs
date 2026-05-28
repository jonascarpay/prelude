#[derive(Debug, Clone, Copy)]
pub struct Unorm8 {
    unscaled: u8,
}

impl Unorm8 {
    pub fn mult_blinn(self, rhs: Self) -> Self {
        let xy = self.unscaled as u16 * rhs.unscaled as u16 + 128;
        Unorm8 {
            unscaled: ((xy + (xy >> 8)) >> 8) as u8,
        }
    }
    pub fn mult_ref(self, rhs: Self) -> Self {
        let xy = self.unscaled as u16 * rhs.unscaled as u16;
        Unorm8 {
            unscaled: ((xy + 127) / 255) as u8,
        }
    }
}
