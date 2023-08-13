#![allow(dead_code)]

use bevy_ecs::{
    prelude::Entity,
    query::With,
    system::{Commands, Query, Res, ResMut},
    world::{self, World},
};
use macroquad::{prelude::Vec2, window::screen_height};

use std::{error::Error, io};
use tracing::{debug, error, info, span, warn, Level};

use rs_nonamerl_core::{
    prelude::{
        EntityAction, EntityActionQueue, EntityActivatorFunctionResult, EntityQueue, FovOccluder,
        GameMap, KeyInput, MapCommand, MapCommands, MoveActionParams, TestCamera2D, UserInput,
        Walkable,
    },
    IntVector2,
};

use crate::{
    components::{Health, MoveIntent, Player, Position},
    tiles::TestTile,
    FovData,
};

pub fn update_player_position(
    user_input: Res<UserInput>,
    // mut action_queue: ResMut<EntityActionQueue>,
    _game_map: Res<GameMap<TestTile>>,
    player_query: Query<(Entity, &Position), With<Player>>,
    mut commands: Commands,
) {
    let _span = tracy_client::span!();
    // let mut position = player_query.single_mut();
    let (player_id, position) = player_query.single();
    let mut dx = IntVector2::default();
    if user_input.key_input == KeyInput::Right {
        dx.x += 1;
    }

    if user_input.key_input == KeyInput::Left {
        dx.x -= 1;
    }

    if user_input.key_input == KeyInput::Up {
        dx.y -= 1;
    }

    if user_input.key_input == KeyInput::Down {
        dx.y += 1;
    }

    if dx != IntVector2::default() {
        println!(
            "target: {:?}",
            IntVector2::new(position.x + dx.x, position.y + dx.y)
        );
        commands.entity(player_id).insert(MoveIntent {
            target: IntVector2::new(position.x + dx.x, position.y + dx.y),
        });
        // action_queue.add(EntityAction::Move(
        //     MoveActionParams {
        //         dx,
        //         start: IntVector2::new(position.x, position.y),
        //         entity: player_id,
        //         // target_entity: None,
        //         // target_items: vec![],
        //     },
        //     None, // Some(|params| EntityActivatorFunctionResult::Ok),
        // ));
    }
}

pub fn update_camera(
    mut camera: ResMut<TestCamera2D>,
    // user_input: Res<UserInput>,
    // viewport: Res<Viewport>,
    player_query: Query<&Position, With<Player>>,
) {
    let _span = tracy_client::span!("update_camera");
    let position = player_query.single();
    // let mouse_state = user_input.mouse_state;
    // let mouse_position = Vec2::new(mouse_state.x, mouse_state.y);
    // let mouse_world_position = camera.viewport_to_world(mouse_position, &viewport);

    // if user_input.key_input == KeyInput::Right {
    //     // camera.position.x += camera.cell_size.width() as f32;
    //     // camera.move_right(1.);
    //     camera.position.x += 1_f32;
    // }

    // if user_input.key_input == KeyInput::Left {
    //     // camera.position.x -= camera.cell_size.width() as f32;
    //     // camera.move_right(-1.);
    //     camera.position.x -= 1_f32;
    // }

    // if user_input.key_input == KeyInput::Up {
    //     camera.position.y -= 1_f32;
    // }

    // if user_input.key_input == KeyInput::Down {
    //     camera.position.y += 1_f32;
    // }
    let alpha = 0.99;

    camera.position =
        camera.position * alpha + (1. - alpha) * Vec2::new(position.x as f32, position.y as f32);

    // if user_input.mouse_state.left_button {
    //     camera.center_on_world_point(mouse_world_position, &viewport);
    // }

    // let zoom = user_input.mouse_state.scroll;
    // if zoom != 0.0 {
    //     // println!("zoom: {:?}", zoom);
    //     let zoom_scale = 1.0 + zoom / (10.0 * screen_height());
    //     camera.zoom_scale *= zoom_scale;
    //     // camera.center_on_fixed_world_point(world_x, world_y, &viewport)
    // }
    camera.update();
    //let camera_pos = camera.position.clone();
}

pub fn update_fov(
    mut fov_data: ResMut<FovData>,
    mut game_map: ResMut<GameMap<TestTile>>,
    player_query: Query<&Position, With<Player>>,
    mut commands: ResMut<MapCommands>,
) {
    let _span = tracy_client::span!("update_fov");
    let position = player_query.single();
    let fov_size = fov_data.fov_size;
    let start_pos = IntVector2::new(position.x, position.y);
    // loop over the border of a 5x5 grid centerd in the mouse position
    for i in -fov_size..=fov_size {
        for j in -fov_size..=fov_size {
            if i == fov_size || i == -fov_size || j == fov_size || j == -fov_size {
                let target = IntVector2::new(position.x + i, position.y + j);
                let path = game_map.line(start_pos, target);
                // println!("target: {:?}", target);
                'outer: for tile in path.iter().map(|v| (v, game_map.get(v.x, v.y)))
                //.take_while(|x| x.1.is_some())
                {
                    let (p, tile) = tile;
                    // print!("p: {:?}, ", p);
                    if fov_data.current_fov_cells.contains(p) {
                        // println!("cell already in fov");
                        continue;
                    }

                    let t = game_map.get(p.x, p.y);
                    if let Some(tile) = t {
                        // let tile = tile.unwrap();

                        if !(tile.block_visibility() == TestTile::BLOCKED) {
                            commands.add(MapCommand::SetVisited(*p, true));
                            fov_data.current_fov_cells.insert(*p);
                        } else {
                            // println!("tile blocked");
                            break 'outer;
                        }
                    } else {
                        // println!("tile not found");
                        break 'outer;
                    }
                }
            }

            // map_batch.push(RenderOp::HighlightCell(coords.x(), coords.y()));
        }
    }

    let fov_cells_to_remove = fov_data.fov_cells.difference(&fov_data.current_fov_cells);
    let fov_cells_to_add = fov_data.current_fov_cells.difference(&fov_data.fov_cells);

    commands.add_all(
        fov_cells_to_remove
            .map(|v| MapCommand::SetVisible(*v, false))
            .collect(),
    );

    commands.add_all(
        fov_cells_to_add
            .map(|v| MapCommand::SetVisible(*v, true))
            .collect(),
    );

    fov_data.fov_cells = fov_data.current_fov_cells.clone();
    fov_data.current_fov_cells.clear();
    commands.process_commands(&mut game_map);
}
