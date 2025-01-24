use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub(crate) a: Vec2,
    pub(crate) b: Vec2
}

impl Line {
    pub fn new(a: Vec2, b: Vec2) -> Line {
        Line {
            a,
            b,
        }
    }

    pub fn as_vec4(&self) -> Vec4 {
        Vec4::new(self.a.x, self.a.y, self.b.x, self.b.y)
    }
}