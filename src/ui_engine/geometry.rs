#[derive(Debug, Clone, Copy)]

pub struct UIPoint {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct UISize {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct UIRect {
    pub(crate) origin: UIPoint,
    pub(crate) size: UISize,
}

impl UIRect {
    pub(crate) fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        return Self {
            origin: UIPoint { x, y },
            size: UISize { width: w, height: h },
        };
    }
    pub(crate) fn x(&self) -> u32{
         self.origin.x
    }
    pub(crate) fn y(&self) -> u32{
         self.origin.y
    }
    pub(crate) fn w(&self) -> u32{
         self.size.width
    }
    pub(crate) fn h(&self) -> u32{
         self.size.height
    }
    pub(crate) fn contains(&self, x: u32, y: u32) -> bool{
        self.origin.x < x && x < (self.origin.x + self.size.width) && self.origin.y < y && y < (self.origin.y + self.size.height)
    }

}