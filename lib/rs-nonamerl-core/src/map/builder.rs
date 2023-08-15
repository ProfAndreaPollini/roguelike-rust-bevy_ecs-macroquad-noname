use std::collections::{HashMap, HashSet};

use rand::seq::{IteratorRandom, SliceRandom};

use crate::{prelude::Tile, IntExtent2, IntVector2};

use super::{GameMap, Room};

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
    pub extent: IntExtent2,
    pub rooms: Vec<Room>,
    /// The different types of tiles that can be used to build the map.
    pub(super) tiles: HashMap<String, T>,
}

impl<T: Tile> MapBuilder<T> {
    pub fn new(extent: IntExtent2) -> Self {
        Self {
            map: GameMap::<T>::new(),
            tiles: HashMap::new(),
            rooms: Vec::new(),
            extent,
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

#[derive(Debug, Clone)]
pub struct RandomWalkBuilder<T>
where
    T: Tile,
{
    start_pos: IntVector2,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Tile> RandomWalkBuilder<T> {
    pub fn new(start_pos: IntVector2) -> Self {
        Self {
            start_pos,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Tile> MapBuilderAlgorithm<T> for RandomWalkBuilder<T> {
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T> {
        let mut rng = rand::thread_rng();
        // let _pos = self.start_pos;

        let mut current_pos = self.start_pos;

        let mut visited = HashSet::<IntVector2>::new();
        let directions = ["up", "down", "left", "right"];
        // generate a random walk
        while visited.len() < 100 {
            let mut next_pos = current_pos;

            // randomly choose a direction
            let direction = directions.choose(&mut rng).unwrap();
            //let direction = directions[dir];

            match *direction {
                "up" => next_pos.x -= 1,
                "down" => next_pos.y += 1,
                "left" => next_pos.x -= 1,
                "right" => next_pos.x += 1,
                _ => {}
            }

            if !visited.insert(next_pos) {
                // select random element from visited
                current_pos = *visited.iter().choose(&mut rng).unwrap();
            } else {
                current_pos = next_pos;
            }
        }
        // println!("visited: {:?}", visited);
        //map_builder.map_tiles.tiles = visited.clone();
        visited.iter().for_each(|pos| {
            let tile = map_builder.tiles.get("floor").unwrap().clone();

            map_builder.map.set(pos.x, pos.y, tile);
        });
        // println!("map_tiles: {:?}", map_builder.map_tiles.tiles);

        map_builder
    }
}
