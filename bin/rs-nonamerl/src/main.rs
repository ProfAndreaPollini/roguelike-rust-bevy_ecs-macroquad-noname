use std::collections::HashSet;

use bevy_ecs::{
    prelude::{Schedule, World},
    query::With,
    schedule::IntoSystemConfigs,
    system::{Query, Res, ResMut, Resource},
};
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};
use rs_nonamerl_core::{
    prelude::{Camera, TestCamera2D, *},
    Dimension2, IntExtent2, IntVector2,
};

use noise::{utils::NoiseMapBuilder, Fbm, Perlin};
mod components;
mod tiles;

use components::*;
use tiles::*;

mod systems;

use systems::*;

// fn highlight_mouse_pointer(
//     user_input: Res<UserInput>,
//     viewport: Res<Viewport>,
//     camera: Res<TestCamera2D>,
// ) {
//     if !viewport.contains_screen_point(user_input.mouse_state.x, user_input.mouse_state.y) {
//         return;
//     }
//     let mouse_state = user_input.mouse_state;
//     let mouse_cell = camera
//         .cell_of(camera.viewport_to_world(Vec2::new(mouse_state.x, mouse_state.y), &viewport))
//         .as_vec2();

//     draw_circle(mouse_state.x, mouse_state.y, 15.0, YELLOW);
//     if viewport.contains_screen_point(mouse_state.x, mouse_state.y) {
//         let world_mouse_pos =
//             camera.viewport_to_world(Vec2::new(mouse_state.x, mouse_state.y), &viewport);
//         draw_text(
//             &format!("{:?}", mouse_cell),
//             mouse_state.x + 20.0,
//             mouse_state.y,
//             20.0,
//             WHITE,
//         );
//     }

//     let (viewport_camera_pos_x, viewport_camera_pos_y) =
//         camera.world_to_viewport(camera.position, &viewport).into();

//     draw_circle(
//         viewport_camera_pos_x,
//         viewport_camera_pos_y,
//         5.,
//         Color {
//             r: 0.0,
//             g: 0.,
//             b: 1.,
//             a: 1.,
//         },
//     );
// }

// fn debug_grid_overlay(
//     viewport: Res<Viewport>,
//     camera: Res<TestCamera2D>,
//     game_map: Res<GameMap<TestTile>>,
// ) {
//     let visibile_cells = camera.get_visibile_extent(&viewport, 12, 12);
//     // println!("visibile_cells: {:?}", visibile_cells);
//     let renderer = Renderer::from_map_cell_size(camera.cell_size);
//     let mut map_batch = Vec::<RenderOp<TestTile>>::new();
//     let origin = camera.world_to_viewport(Vec2::default(), &viewport);
//     let (origin_x, origin_y) = origin.into();

//     draw_circle(origin_x, origin_y, 5., Color::new(1., 0., 0., 1.));
// }

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_width: 2000,
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

#[macroquad::main(window_conf)]
async fn main() {
    init_log();
    let sprite_container = SpriteContainer::from_config("data/config/sprites.json").await;
    let font = load_ttf_font("assets/fonts/dealerplate_california.otf")
        .await
        .unwrap();

    let viewport = Viewport {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 600.0,
    };

    let mut camera =
        TestCamera2D::from_viewport(Vec2::default(), &viewport, 2.0, Dimension2::new(12, 12));

    let mut world = World::default();
    world.insert_resource(UserInput::default());
    world.insert_resource(viewport);
    world.insert_resource(camera);
    world.insert_resource(FovData::default());
    // world.insert_resource(game_map);
    world.insert_resource(sprite_container);
    world.insert_resource(MapCommands::default());
    world.insert_resource(EntityActionQueue::default());

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

    let mut input_schedule = Schedule::default();
    input_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    // Add our system to the schedule
    // input_schedule.add_systems(test);

    input_schedule.add_systems(update_user_input);

    // Create a new Schedule, which defines an execution strategy for Systems
    let mut update_schedule = Schedule::default();
    update_schedule.add_systems(update_player_position);
    update_schedule.add_systems(process_actions.after(update_player_position));

    update_schedule.add_systems(update_camera);
    update_schedule.add_systems(
        update_fov
            .after(update_camera)
            .after(update_player_position),
    );

    let mut draw_schedule = Schedule::default();
    draw_schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    // Add our system to the schedule
    draw_schedule.add_systems(debug_ui);
    draw_schedule.add_systems(draw_player.after(draw_game_map));
    draw_schedule.add_systems(draw_game_map);
    draw_schedule.add_systems(draw_fov.after(draw_player));
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
