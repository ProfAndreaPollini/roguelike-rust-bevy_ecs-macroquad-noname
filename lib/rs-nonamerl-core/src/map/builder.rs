use std::collections::HashMap;

use bevy_ecs::system::In;

use crate::{prelude::Tile, Dimension2, IntExtent2};

use super::GameMap;

pub trait MapBuilderAlgorithm<T: Tile> {
    /// Builds a map using the given `MapBuilder`.
    ///
    /// # Arguments
    ///
    /// * `map_builder` - A mutable reference to a `MapBuilder` instance.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `MapBuilder` instance.
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T>;
}

#[derive(Clone, Debug)]
pub struct MapBuilder<T: Tile> {
    /// The extent of the map
    pub map: GameMap<T>,
    // pub extent: IntExtent2,
    // pub rooms: Vec<Room>,
    /// The different types of tiles that can be used to build the map.
    pub(super) tiles: HashMap<String, T>,
}

impl<T: Tile> MapBuilder<T> {
    pub fn new(size: Dimension2) -> Self {
        Self {
            map: GameMap::<T>::new(size),
            tiles: HashMap::new(),
            // extent,
        }
    }

    pub fn add_tile(&mut self, name: String, tile: T) {
        self.tiles.insert(name, tile);
    }

    pub fn get_tile(&self, name: &str) -> Option<&T> {
        self.tiles.get(name)
    }

    pub fn build_step(&mut self, algorithm: &impl MapBuilderAlgorithm<T>) -> &mut Self {
        algorithm.build(self)
    }

    pub fn build(&mut self) -> GameMap<T> {
        self.map.clone()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FillWithFloorBuilderAlgo<'a, T>
where
    T: Clone,
{
    extent: IntExtent2,
    tile_name: &'a str,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Clone> FillWithFloorBuilderAlgo<'a, T> {
    pub fn new(extent: IntExtent2, tile_name: &'a str) -> Self {
        Self {
            extent,
            tile_name,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T: Tile> MapBuilderAlgorithm<T> for FillWithFloorBuilderAlgo<'a, T> {
    fn build<'b>(&self, map_builder: &'b mut MapBuilder<T>) -> &'b mut MapBuilder<T> {
        for x in self.extent.left()..self.extent.right() {
            for y in self.extent.top()..self.extent.bottom() {
                // let tile = match (x + y).abs().rem_euclid(2) == 0 {
                //     true => map_builder.tiles[self.tile_name].clone(),
                //     false => map_builder.tiles["wall"].clone(),
                // };
                let tile = map_builder.tiles[self.tile_name].clone();
                map_builder.map.set(x, y, tile);
            }
        }

        map_builder
    }
}
