use bevy_ecs::world::World;
use noise::{Fbm, Perlin};
use rs_nonamerl_core::{
    prelude::{BuilderAlgoWithNoise, FillWithFloorBuilderAlgo, MapBuilder},
    Dimension2, IntExtent2,
};

use crate::tiles::{TestTile, TileKind};

pub fn generate_world_map(world: &mut World) {
    println!("generate_world_map");
    let mut map_builder = MapBuilder::<TestTile>::new(Dimension2::new(100, 100));
    map_builder.add_tile(
        "floor".to_owned(),
        TestTile {
            kind: TileKind::Floor,
            visible: true,
            ..Default::default()
        },
    );

    map_builder.add_tile(
        "wall".to_owned(),
        TestTile {
            kind: TileKind::Wall("wall"),
            visible: true,
            ..Default::default()
        },
    );
    map_builder.add_tile(
        "wall2".to_owned(),
        TestTile {
            kind: TileKind::Wall("wall2"),
            visible: true,
            ..Default::default()
        },
    );

    let mut noise = Fbm::<Perlin>::default();

    noise.octaves = 6;
    noise.frequency = 1.;
    noise.lacunarity = 2.;
    noise.persistence = 0.5;

    let f = |x: i32, y: i32, value: f64| {
        // println!("x: {}, y: {}, value: {}", x, y, value);
        if value > 0. {
            Some("wall2".to_owned())
        } else {
            None
        }
    };

    let game_map = map_builder
        .build_step(&FillWithFloorBuilderAlgo::new(
            IntExtent2::new(-50, -50, 100, 100),
            "floor",
        ))
        .build_step(&BuilderAlgoWithNoise::new(
            &noise,
            f,
            IntExtent2::new(0, 0, 100, 100),
        ))
        .build();

    world.insert_resource(game_map);
}
