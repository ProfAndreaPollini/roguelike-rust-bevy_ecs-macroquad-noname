#![allow(dead_code)]
use bevy_ecs::{
    query::With,
    system::{Commands, Query, Res, ResMut},
    world::World,
};
use macroquad::prelude::KeyCode;
use noise::{Fbm, Perlin};
use rand::{seq::IteratorRandom, Rng};
use rs_nonamerl_core::{
    prelude::{
        BuilderAlgoWithNoise, FillWithFloorBuilderAlgo, GameMap, KeyInput, MapBuilder, RoomBuilder,
    },
    IntExtent2, IntVector2,
};

use crate::{
    components::{
        DrinkEffect, Enemy, Health, Interaction, Interactions, Item, ItemKind, ModHealth, Player,
        Position, SpriteDrawInfo, UseKind,
    },
    resources::{GameContext, GameState},
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

pub fn spawn_enemies(
    level_data: Res<LevelData>,
    mut commands: Commands,
    player_query: Query<&Position, With<Player>>,
) {
    println!("spawn_enemies");
    let rooms = &level_data.rooms;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut position = player_query.single().clone();
    // let spawn_point = *rooms
    //     .iter()
    //     .skip(1)
    //     .choose(&mut rng)
    //     .unwrap()
    //     .interior_cells()
    //     .iter()
    //     .choose(&mut rng)
    //     .unwrap();

    position.x += rng.gen_range(-5..5);
    position.y += rng.gen_range(-5..5);

    commands.spawn((
        Position {
            x: position.x,
            y: position.y,
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

pub fn spawn_items(
    level_data: Res<LevelData>,
    game_map: Res<GameMap<TestTile>>,
    mut commands: Commands,
    player_query: Query<&Position, With<Player>>,
    mut game_ctx: ResMut<GameContext>,
) {
    // let rooms = &level_data.rooms;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut position = player_query.single().clone();
    // let spawn_point = *rooms
    //     .iter()
    //     .skip(1)
    //     .choose(&mut rng)
    //     .unwrap()
    //     .interior_cells()
    //     .iter()
    //     .choose(&mut rng)
    //     .unwrap();

    position.x += rng.gen_range(-1..1);
    position.y += rng.gen_range(-1..1);

    let items = game_map.items(IntVector2::new(position.x, position.y));
    tracing::info!("items: {:?}", items);

    let interactions = Interactions {
        interactions: vec![
            Interaction::new(KeyInput::Key(KeyCode::E), UseKind::Pick),
            Interaction::new(
                KeyInput::Key(KeyCode::Y),
                UseKind::Drink(DrinkEffect {
                    health: 10,
                    stamina: 5,
                    mana: 5,
                }),
            ),
        ],
    };

    let item_id = commands
        .spawn((
            Position {
                x: position.x,
                y: position.y,
            },
            Item {
                name: "basic potion".to_owned(),
                kind: ItemKind::Potion,
            },
            SpriteDrawInfo {
                sprite_info: "item01",
            },
            ModHealth { amount: 10 },
            interactions,
        ))
        .id();

    game_map.add_item(IntVector2::new(position.x, position.y), item_id);

    tracing::info!(
        "items after add: {:?}",
        game_map.items(IntVector2::new(position.x, position.y))
    );
    game_ctx.state = GameState::PlayGame;
}
