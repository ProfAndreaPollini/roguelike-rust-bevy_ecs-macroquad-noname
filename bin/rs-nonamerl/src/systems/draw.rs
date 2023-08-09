use bevy_ecs::{
    query::With,
    system::{Query, Res},
};
use macroquad::prelude::Color;
use rs_nonamerl_core::prelude::{
    GameMap, RenderOp, Renderer, SpriteContainer, TestCamera2D, Viewport,
};

use crate::{
    components::{Player, Position, SpriteDrawInfo},
    tiles::TestTile,
    FovData,
};

pub fn draw_game_map(
    game_map: Res<GameMap<TestTile>>,
    camera: Res<TestCamera2D>,
    viewport: Res<Viewport>,
    sprites: Res<SpriteContainer>,
) {
    let visibile_cells = camera.visible_tiles_extent;

    let renderer = Renderer::from_map_cell_size(camera.cell_size);
    let mut map_batch = Vec::<RenderOp<TestTile>>::new();

    for x in visibile_cells.left()..=visibile_cells.right() {
        for y in visibile_cells.top()..=visibile_cells.bottom() {
            if let Some(tile) = game_map.get(x, y) {
                map_batch.push(RenderOp::DrawTile(x, y, tile));
            }
        }
    }

    renderer.batch_render(&camera, &viewport, &sprites, &map_batch);
}

pub fn draw_player(
    player_query: Query<(&Position, &SpriteDrawInfo), With<Player>>,
    camera: Res<TestCamera2D>,
    viewport: Res<Viewport>,
    sprites: Res<SpriteContainer>,
) {
    let (position, player_draw_info) = player_query.single();

    let renderer = Renderer::from_map_cell_size(camera.cell_size);
    let mut player_batch = Vec::<RenderOp<TestTile>>::new();

    player_batch.push(RenderOp::DrawEntity(
        position.x,
        position.y,
        player_draw_info.sprite_info,
    ));

    renderer.batch_render(&camera, &viewport, &sprites, &player_batch);
}

pub fn draw_fov(fov_data: Res<FovData>, camera: Res<TestCamera2D>, viewport: Res<Viewport>) {
    let renderer = Renderer::from_map_cell_size(camera.cell_size);
    let mut fov_batch = Vec::<RenderOp<TestTile>>::new();

    // for cell in fov_data.current_fov_cells.iter() {
    //     fov_batch.push(RenderOp::HighlightCell(cell.x(), cell.y()));
    // }

    renderer.render_fov(
        fov_data.fov_cells.iter().copied().collect(),
        &camera,
        Color::new(1.0, 1.0, 1.0, 0.4),
    );
}
