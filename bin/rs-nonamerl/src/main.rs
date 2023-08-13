#![allow(dead_code)]

use std::collections::HashSet;

use bevy_ecs::{
    prelude::{Schedule, World},
    schedule::IntoSystemConfigs,
    system::Resource,
};
use macroquad::prelude::*;

use tracing::Level;

use rs_nonamerl_core::{
    prelude::{TestCamera2D, *},
    Dimension2, IntVector2,
};

mod components;
mod tiles;

use components::*;

mod systems;

use systems::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Noname RL".to_owned(),
        fullscreen: false,
        window_width: 1900,
        window_height: 800,
        ..Default::default()
    }
}

fn init_log() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

#[derive(Debug, Clone, Resource)]
pub struct FovData {
    pub fov_size: i32,
    pub fov_cells: HashSet<IntVector2>,
    pub current_fov_cells: HashSet<IntVector2>,
}

impl Default for FovData {
    fn default() -> Self {
        Self {
            fov_size: 5,
            fov_cells: HashSet::new(),
            current_fov_cells: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct LevelData {
    pub rooms: Vec<Room>,
    // pub corridors: Vec<Vec<IntVector2>>,
}

#[macroquad::main(window_conf)]
async fn main() {
    init_log();
    let _client = tracy_client::Client::start();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let sprite_container = SpriteContainer::from_config("data/config/sprites.json").await;
    let _font = load_ttf_font("assets/fonts/dealerplate_california.otf")
        .await
        .unwrap();

    let viewport = Viewport {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 600.0,
    };

    let camera =
        TestCamera2D::from_viewport(Vec2::default(), &viewport, 1.2, Dimension2::new(12, 12));

    let mut world = World::default();
    world.insert_resource(UserInput::default());
    world.insert_resource(viewport);
    world.insert_resource(camera);
    world.insert_resource(FovData::default());
    // world.insert_resource(game_map);
    world.insert_resource(sprite_container);
    world.insert_resource(MapCommands::default());
    world.insert_resource(EntityActionQueue::default());
    // world.insert_resource(Vec::<Room>::default());
    world.insert_resource(LevelData::default());

    // create player
    world.spawn((
        Position { x: 0, y: 0 },
        Player {},
        SpriteDrawInfo {
            sprite_info: "hero",
        },
        Health {
            current: 100,
            max: 100,
        },
    ));

    let mut setup_schedule = Schedule::default();
    setup_schedule.add_systems(generate_world_map);
    setup_schedule.add_systems(spawn_enemies.after(generate_world_map));

    let mut input_schedule = Schedule::default();
    input_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    // Add our system to the schedule
    // input_schedule.add_systems(test);

    input_schedule.add_systems(update_user_input);

    // Create a new Schedule, which defines an execution strategy for Systems
    let mut update_schedule = Schedule::default();
    update_schedule.add_systems(update_player_position);
    // update_schedule.add_systems(process_actions.after(update_player_position));

    update_schedule.add_systems(update_camera);
    update_schedule.add_systems(
        update_fov
            .after(update_camera)
            .after(update_player_position),
    );
    update_schedule.add_systems(move_intent_system);

    let mut draw_schedule = Schedule::default();
    draw_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    // Add our system to the schedule
    draw_schedule.add_systems(debug_ui);
    draw_schedule.add_systems(draw_player.after(draw_game_map));
    draw_schedule.add_systems(draw_enemies.after(draw_game_map));
    draw_schedule.add_systems(draw_game_map);
    // draw_schedule.add_systems(draw_fov.after(draw_player));
    // draw_schedule.add_systems(highlight_mouse_pointer);
    // draw_schedule.add_systems(debug_grid_overlay);

    // Run the setup schedule once
    setup_schedule.run(&mut world);
    loop {
        // Run the schedule once. If your app has a "loop", you would run this once per loop

        clear_background(DARKBROWN);
        input_schedule.run(&mut world);
        update_schedule.run(&mut world);
        draw_schedule.run(&mut world);

        next_frame().await
    }
}
