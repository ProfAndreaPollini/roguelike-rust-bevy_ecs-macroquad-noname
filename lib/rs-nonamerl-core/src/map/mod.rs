use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

use bevy_ecs::{prelude::Entity, system::Resource};

use crate::{
    prelude::LatticeGrid2D, prelude::Plane, tile::Tile, Dimension2, IntExtent2, IntVector2,
};

mod builder;
mod command;

mod noise_builder;

pub use builder::*;
pub use command::*;

pub use noise_builder::*;

#[derive(Debug, Clone, Resource)]
pub struct GameMap<T: Tile> {
    pub grid: Arc<RwLock<LatticeGrid2D<T>>>,
    pub size: Dimension2,
}

impl<T: Tile> GameMap<T> {
    pub fn new(size: Dimension2) -> Self {
        Self {
            grid: Arc::new(RwLock::new(LatticeGrid2D::new())),
            size,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<T> {
        // let binding = self.grid.borrow();
        self.grid
            .read()
            .unwrap()
            .at(IntVector2::new(x, y))
            .map(|tile| (*tile).clone())
    }

    pub fn get_position(&self, position: IntVector2) -> Option<T> {
        // let binding = self.grid.borrow();
        self.grid
            .read()
            .unwrap()
            .at(position)
            .map(|tile| (*tile).clone())
    }

    pub fn set(&self, x: i32, y: i32, tile: T) {
        self.grid.write().unwrap().put(IntVector2::new(x, y), tile);
    }

    pub fn size(&self) -> Dimension2 {
        self.size
    }

    pub fn len(&self) -> usize {
        self.grid.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.grid.read().unwrap().is_empty()
    }

    fn add_item(&self, position: IntVector2, item: Entity) {
        if let Some(tile) = self.grid.write().unwrap().at_mut(position) {
            tile.add_item(item);
        }
    }

    fn remove_item(&self, position: IntVector2, item: Entity) {
        if let Some(tile) = self.grid.write().unwrap().at_mut(position) {
            tile.remove_item(item);
        }
    }

    pub fn iter_over_visible_tiles<'a>(&'a self, extent: &'a IntExtent2) -> MapVisibleTilesIter<T> {
        MapVisibleTilesIter::new(self, extent)
    }

    pub fn set_visited(&self, position: IntVector2, visited: bool) {
        if let Some(tile) = self.grid.write().unwrap().at_mut(position) {
            tile.set_visited(visited);
        }
    }

    pub fn set_visible(&self, position: IntVector2, visible: bool) {
        if let Some(tile) = self.grid.write().unwrap().at_mut(position) {
            tile.set_visible(visible);
        }
    }
    pub fn line(&self, start: IntVector2, end: IntVector2) -> Vec<IntVector2> {
        self.grid.read().unwrap().line(start, end)
    }
}

pub struct MapVisibleTilesIter<'a, T: Tile> {
    map: &'a GameMap<T>,
    extent: &'a IntExtent2,
    current: IntVector2,
}

impl<'a, T: Tile> MapVisibleTilesIter<'a, T> {
    pub fn new(map: &'a GameMap<T>, extent: &'a IntExtent2) -> Self {
        Self {
            map,
            extent,
            current: IntVector2::new(extent.left(), extent.top()),
        }
    }
}

impl<'a, T: Tile> Iterator for MapVisibleTilesIter<'a, T> {
    type Item = (IntVector2, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.extent.bottom() {
            return None;
        }

        let mut pos = self.current;

        self.current.x += 1; // = IntVector2::new(self.current.x + 1, self.current.y);

        if self.current.x >= self.extent.right() {
            self.current.x = self.extent.left(); // = IntVector2::new(self.extent.left(), self.current.y + 1);
            self.current.y += 1;
        }
        // dbg!(self.current, self.extent);

        if let Some(tile) = self.map.get(pos.x, pos.y) {
            // if tile.is_visible() {
            dbg!(format!("returning tile: {:?}", tile.clone()));
            return Some((pos, tile));
            // }
        } else {
            dbg!(format!("no tile at: {:?}", pos));
        }

        self.next()
    }
}
