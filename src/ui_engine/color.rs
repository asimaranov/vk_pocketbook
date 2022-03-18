#[derive(Debug, Copy, Clone)]
pub struct UIColor(pub i32);

impl UIColor {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(((blue as i32) << 16) + ((green as i32) << 8) + red as i32)
    }

    pub const fn gs(intensity: u8) -> Self {
        Self(intensity as i32 * 0x010101)
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);

    pub const WHITE:Self = Self::rgb(255, 255, 255);
}