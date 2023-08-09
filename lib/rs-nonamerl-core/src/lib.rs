#![allow(dead_code)]
use macroquad::prelude::{IVec2, UVec2};

mod geometry;

mod action;
mod camera;
mod map;
mod renderer;
mod sprite;
mod tile;
mod user_input;

pub mod prelude {
    pub use crate::action::*;
    pub use crate::camera::*;
    pub use crate::geometry::*;
    pub use crate::map::*;
    pub use crate::renderer::*;
    pub use crate::sprite::*;
    pub use crate::tile::*;
    pub use crate::user_input::*;

    pub use crate::camera::Camera;
    pub use crate::tile::FovOccluder;
}

// use prelude::*;

pub type IntVector2 = IVec2;

#[derive(Debug, Copy, Clone, Default)]
pub struct Dimension2(UVec2);

impl Dimension2 {
    pub fn new(width: u32, height: u32) -> Self {
        Self(UVec2::new(width, height))
    }

    pub fn width(&self) -> u32 {
        self.0.x
    }

    pub fn height(&self) -> u32 {
        self.0.y
    }

    pub fn mut_width(&mut self) -> &mut u32 {
        &mut self.0.x
    }

    pub fn mut_height(&mut self) -> &mut u32 {
        &mut self.0.y
    }
}

#[derive(Debug, Copy, Clone, Default)]
/// A generic struct representing an AABB dimension in 2D space.
///
/// This struct is used to represent the size of a cell in a grid.
///
///
/// # Examples
///
///     
pub struct IntExtent2(IntVector2, Dimension2);

impl IntExtent2 {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self(IntVector2::new(x, y), Dimension2::new(width, height))
    }

    pub fn left(&self) -> i32 {
        self.0.x
    }

    pub fn right(&self) -> i32 {
        self.0.x + self.1.width() as i32
    }

    pub fn top(&self) -> i32 {
        self.0.y
    }

    pub fn bottom(&self) -> i32 {
        self.0.y + self.1.height() as i32
    }

    pub fn width(&self) -> u32 {
        self.1.width()
    }

    pub fn height(&self) -> u32 {
        self.1.height()
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    /// Returns an iterator over the cells in the extent.
    pub fn iter(&self) -> IntExtent2Iterator {
        IntExtent2Iterator::new(self)
    }
}

pub struct IntExtent2Iterator<'a> {
    extent: &'a IntExtent2,
    current: IntVector2,
}

impl<'a> IntExtent2Iterator<'a> {
    pub fn new(extent: &'a IntExtent2) -> Self {
        Self {
            extent,
            current: extent.0,
        }
    }
}

impl<'a> Iterator for IntExtent2Iterator<'a> {
    type Item = IntVector2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.extent.bottom() {
            return None;
        }

        let result = self.current;

        self.current = IntVector2::new(self.current.x + 1, self.current.y);

        if self.current.x >= self.extent.right() {
            self.current = IntVector2::new(self.extent.left(), self.current.y + 1);
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = IntVector2::new(-1, 2);
        assert_eq!(v.x, -1);
        v.x = 3;
        assert_eq!(v.x, 3);
    }

    #[test]
    fn test_extent2() {
        let e = IntExtent2::new(0, 0, 10, 10);
        assert_eq!(e.left(), 0);
        assert_eq!(e.right(), 10);
        assert_eq!(e.top(), 0);
        assert_eq!(e.bottom(), 10);
        assert_eq!(e.width(), 10);
        assert_eq!(e.height(), 10);
        assert!(e.contains(0, 0));
        assert!(e.contains(9, 9));
        assert!(!e.contains(10, 10));
    }
}
