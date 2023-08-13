#![allow(dead_code)]
use bevy_ecs::{
    system::{Commands, Res},
    world::World,
};
use noise::{Fbm, Perlin};
use rand::seq::IteratorRandom;
use rs_nonamerl_core::{
    prelude::{BuilderAlgoWithNoise, FillWithFloorBuilderAlgo, MapBuilder, RoomBuilder},
    IntExtent2,
};

use crate::{
    components::{Enemy, Health, Position, SpriteDrawInfo},
    tiles::{TestTile, TileKind},
    LevelData,
};

pub fn generate_world_map(world: &mut World) {
    println!("generate_world_map");

    let mut map_builder = MapBuilder::<TestTile>::new(IntExtent2::new(-100, -100, 200, 200));
    map_builder.add_tile(
        "floor".to_owned(),
        TestTile {
            kind: TileKind::Floor,
            visible: false,
            ..Default::default()
        },
    );

    map_builder.add_tile(
        "wall".to_owned(),
        TestTile {
            kind: TileKind::Wall("wall"),
            visible: false,
            ..Default::default()
        },
    );
    map_builder.add_tile(
        "wall2".to_owned(),
        TestTile {
            kind: TileKind::Wall("wall2"),
            visible: false,
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
            IntExtent2::new(-10, -10, 50, 50),
            "floor",
        ))
        .build_step(&BuilderAlgoWithNoise::new(
            &noise,
            f,
            IntExtent2::new(0, 0, 100, 100),
        ))
        // .build_step(&RandomWalkBuilder::new(IntVector2::new(0, 0)))
        .build_step(&RoomBuilder::new())
        .build();

    // level_data.rooms = map_builder.rooms.clone();

    let level_data = LevelData {
        rooms: map_builder.rooms.clone(),
    };
    world.insert_resource(game_map);
    world.insert_resource(level_data);
}

pub fn spawn_enemies(level_data: Res<LevelData>, mut commands: Commands) {
    println!("spawn_enemies");
    let rooms = &level_data.rooms;
    let mut rng = rand::thread_rng();
    let spawn_point = *rooms
        .iter()
        .skip(1)
        .choose(&mut rng)
        .unwrap()
        .interior_cells()
        .iter()
        .choose(&mut rng)
        .unwrap();
    commands.spawn((
        Position {
            x: spawn_point.x,
            y: spawn_point.y,
        },
        Enemy {},
        SpriteDrawInfo {
            sprite_info: "enemy01",
        },
        Health {
            current: 100,
            max: 100,
        },
    ));
}
