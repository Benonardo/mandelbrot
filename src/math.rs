use std::ops::Sub;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2I {
    pub x: i32,
    pub y: i32,
}

impl Vec2I {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
