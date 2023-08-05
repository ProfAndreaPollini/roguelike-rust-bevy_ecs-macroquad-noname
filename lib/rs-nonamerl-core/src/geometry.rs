#![allow(dead_code)]

use morton_encoding::{morton_decode, morton_encode};
use std::collections::{HashMap, HashSet};

use crate::IntVector2;

/// A generic struct representing a Plane (Lattice) in 2D space. It is a plane where coordinates
/// are integers and the content of each cell is generic of type T.
pub trait Plane<T> {
    fn at(&self, position: IntVector2) -> Option<&T>;
    fn at_mut(&mut self, position: IntVector2) -> Option<&mut T>;
    // TODO: Think about this. Maybe it is better to use only at_mut and return a reference to the cell.
    fn put(&mut self, pos: IntVector2, value: T);
    fn line(&self, start: IntVector2, end: IntVector2) -> Vec<IntVector2>;
    fn neighbors(&self, pos: IntVector2) -> Vec<IntVector2>; // TODO: Maybe it is better to return a slice
}

/// A encoder for points on a 2d plane. It uses a morton encoding to encode the points.
pub trait PositionEncoder {
    fn encode(&self) -> u64;
    fn translate(val: i32) -> u32;
}

pub trait PositionDecoder {
    fn decode(self) -> IntVector2;
    fn untranslate(val: u32) -> i32;
}

impl PositionEncoder for IntVector2 {
    fn encode(&self) -> u64 {
        // (v.x() + v.y() * 1000) as u32
        morton_encode([Self::translate(self.x), Self::translate(self.y)])
    }

    #[inline]
    fn translate(val: i32) -> u32 {
        let v = val as i64;
        (v + i32::MAX as i64) as u32
    }
}

impl PositionDecoder for u64 {
    fn decode(self) -> IntVector2 {
        let [x, y] = morton_encoding::morton_decode(self);
        IntVector2::new(Self::untranslate(x), Self::untranslate(y))
    }

    #[inline]
    fn untranslate(val: u32) -> i32 {
        let v = val as i64;
        (v - i32::MAX as i64) as i32
    }
}

#[derive(Clone, Debug)]
pub struct LatticeGrid2D<T>
where
    T: Clone,
{
    data: HashMap<u64, T>,
}

impl<T: Clone> LatticeGrid2D<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<T: Clone> Default for LatticeGrid2D<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> std::fmt::Display for LatticeGrid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("LatticeGrid2D [{:?}]", self.data.len()));

        write!(f, "{}", s)
    }
}

impl<T: Clone> Plane<T> for LatticeGrid2D<T> {
    fn at(&self, position: IntVector2) -> Option<&T> {
        self.data.get(&position.encode())
    }

    fn at_mut(&mut self, position: IntVector2) -> Option<&mut T> {
        self.data.get_mut(&position.encode())
    }

    fn put(&mut self, pos: IntVector2, value: T) {
        self.data.insert(pos.encode(), value);
    }

    fn neighbors(&self, pos: IntVector2) -> Vec<IntVector2> {
        let mut neighbors = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let neighbor = IntVector2::new(pos.x + x, pos.y + y);
                if let Some(_) = self.at(neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }

    fn line(&self, start: IntVector2, end: IntVector2) -> Vec<IntVector2> {
        bresenham_line(start, end)
    }
}

#[inline]
pub fn bresenham_line(
    start: IntVector2,
    end: IntVector2,
    // mut x1: IntVector2,
    // mut y1: IntVector2,
) -> Vec<IntVector2> {
    use std::mem::swap;

    let mut x0 = start.x;
    let mut y0 = start.y;
    let mut x1 = end.x;
    let mut y1 = end.y;

    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    // let reverse_output = x0 > x1;
    let start_x = x0;
    let start_y = y0;
    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2: i32 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    let mut cells: Vec<IntVector2> = vec![];

    let mut x = x0;
    while x <= x1 {
        if steep {
            // image.set(y as usize, x as usize, color).ok();
            cells.push(IntVector2::new(y, x));
        } else {
            // image.set(x as usize, y as usize, color).ok();
            cells.push(IntVector2::new(x, y));
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
        x += 1;
    }
    // println!("cells: {:?}", cells);
    if cells[0].x != start_x && cells[0].y != start_y {
        // println!("cells[0] != x0 || cells[0] != y0");
        cells.reverse();
    }
    cells
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_lattice_grid() {
        use super::*;
        let mut grid = LatticeGrid2D::<i32>::new();
        grid.put(IntVector2::new(0, 0), 1);
        grid.put(IntVector2::new(1, 0), 2);
        grid.put(IntVector2::new(0, 1), 3);
        grid.put(IntVector2::new(1, 1), 4);

        assert_eq!(grid.at(IntVector2::new(0, 0)), Some(&1));
        assert_eq!(grid.at(IntVector2::new(1, 0)), Some(&2));
        assert_eq!(grid.at(IntVector2::new(0, 1)), Some(&3));
        assert_eq!(grid.at(IntVector2::new(1, 1)), Some(&4));

        assert_eq!(grid.at_mut(IntVector2::new(0, 0)), Some(&mut 1));
        assert_eq!(grid.at_mut(IntVector2::new(1, 0)), Some(&mut 2));
        assert_eq!(grid.at_mut(IntVector2::new(0, 1)), Some(&mut 3));
        assert_eq!(grid.at_mut(IntVector2::new(1, 1)), Some(&mut 4));

        assert_eq!(grid.at(IntVector2::new(2, 2)), None);
        assert_eq!(grid.at_mut(IntVector2::new(2, 2)), None);

        assert_eq!(grid.at(IntVector2::new(0, 0)), Some(&1));
        assert_eq!(grid.at(IntVector2::new(1, 0)), Some(&2));
        assert_eq!(grid.at(IntVector2::new(0, 1)), Some(&3));
        assert_eq!(grid.at(IntVector2::new(1, 1)), Some(&4));

        assert_eq!(grid.at_mut(IntVector2::new(0, 0)), Some(&mut 1));
        assert_eq!(grid.at_mut(IntVector2::new(1, 0)), Some(&mut 2));
        assert_eq!(grid.at_mut(IntVector2::new(0, 1)), Some(&mut 3));
        assert_eq!(grid.at_mut(IntVector2::new(1, 1)), Some(&mut 4));

        assert_eq!(grid.at(IntVector2::new(2, 2)), None);
    }
}
