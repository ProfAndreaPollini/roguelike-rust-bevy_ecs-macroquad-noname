use noise::NoiseFn;

use crate::{prelude::Tile, IntExtent2};

use super::{MapBuilder, MapBuilderAlgorithm};

#[derive(Debug, Copy, Clone)]
pub struct BuilderAlgoWithNoise<T, N, F>
where
    T: Tile,
    N: NoiseFn<f64, 2>,
    F: Fn(i32, i32, f64) -> Option<String>,
{
    noise: N,
    f: F,
    extent: IntExtent2,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Tile, N: NoiseFn<f64, 2>, F: Fn(i32, i32, f64) -> Option<String>>
    BuilderAlgoWithNoise<T, N, F>
{
    pub fn new(noise_fn: N, f: F, extent: IntExtent2) -> Self {
        Self {
            noise: noise_fn,
            f,
            extent,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Tile, N: NoiseFn<f64, 2>, F: Fn(i32, i32, f64) -> Option<String>> MapBuilderAlgorithm<T>
    for BuilderAlgoWithNoise<T, N, F>
{
    fn build<'a>(&self, map_builder: &'a mut MapBuilder<T>) -> &'a mut MapBuilder<T> {
        for x in self.extent.left()..self.extent.right() {
            for y in self.extent.top()..self.extent.bottom() {
                let value = self.noise.get([
                    x as f64 / (self.extent.width()) as f64,
                    y as f64 / (self.extent.height()) as f64,
                ]);
                let tile = (self.f)(x, y, value);

                if let Some(tile_name) = tile {
                    map_builder
                        .map
                        .set(x, y, map_builder.tiles[tile_name.as_str()].clone());
                }
            }
        }

        map_builder
    }
}
