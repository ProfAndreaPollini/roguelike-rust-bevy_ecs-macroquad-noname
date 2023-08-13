use bevy_ecs::{
    query::With,
    system::{Query, Res},
    world::World,
};
use macroquad::{
    hash,
    prelude::{vec2, Vec2},
    ui::{root_ui, widgets},
};
use rs_nonamerl_core::prelude::{GameMap, TestCamera2D, UserInput, Viewport};

use crate::{
    components::{Health, Player, Position},
    tiles::TestTile,
};

pub fn debug_ui(
    user_input: Res<UserInput>,
    viewport: Res<Viewport>,
    camera: Res<TestCamera2D>,
    game_map: Res<GameMap<TestTile>>,
    player_query: Query<(&Position, &Health), With<Player>>,
    world: &World,
) {
    tracy_client::Client::running()
        .expect("client must be running")
        .plot(
            tracy_client::plot_name!("entities"),
            world.entities().len() as f64,
        );
    // ui::root_ui().push_skin(&skin1);
    let (position, health) = player_query.single();
    let visibile_cells = camera.visible_tiles_extent;
    // for x in visibile_cells.left()..visibile_cells.right() {
    //     for y in visibile_cells.top()..visibile_cells.bottom() {
    //         let tile = game_map.get(x, y);
    //         if tile.is_none() {
    //             continue;
    //         }
    //         let tile = tile.unwrap();
    //     }
    // }
    let player_tile = game_map.get(position.x, position.y);

    let mouse_pos = Vec2::new(user_input.mouse_state.x, user_input.mouse_state.y);
    // let mouse_world_pos = camera.viewport_to_world(mouse_pos);
    let mouse_tile_pos = camera.viewport_to_tile(mouse_pos);
    let mouse_tile = game_map.get(mouse_tile_pos.x, mouse_tile_pos.y);

    widgets::Window::new(
        hash!(),
        vec2(viewport.x + viewport.width + 100., viewport.y),
        vec2(1020., 800.),
    )
    .movable(false)
    .ui(&mut root_ui(), |ui| {
        ui.label(
            None,
            &format!(
                "Mouse position: ({:?},{:?})",
                user_input.mouse_state.x, user_input.mouse_state.y
            ),
        );

        ui.label(None, "Camera");
        ui.separator();
        ui.label(None, &format!("Camera: {:?}", camera.position));
        ui.label(None, &format!("Camera zoom: {:?}", camera.zoom_scale));
        ui.label(None, &format!("Camera cell size: {:?}", camera.cell_size));
        // ui.label(None, &format!("Camera fov: {:?}", camera.fov));
        ui.label(None, &format!("Camera viewport: {:?}", viewport));
        ui.separator();
        ui.label(None, &format!("GameMap tiles: {:?}", game_map.len()));
        ui.label(
            None,
            &format!("GameMap visible cells: {:?}", visibile_cells),
        );
        ui.label(None, &format!("Player position: {:?}", position));
        ui.label(None, &format!("Player tile: {:?}", player_tile));
        ui.label(None, &format!("Player health: {:?}", health));
        ui.separator();
        ui.label(None, &format!("Mouse tile: {:?}", mouse_tile_pos));
        ui.label(None, &format!("Mouse tile content: {:?}", mouse_tile));
    });
}
