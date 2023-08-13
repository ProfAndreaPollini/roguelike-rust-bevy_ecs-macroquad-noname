use bevy_ecs::system::Resource;
use macroquad::prelude::{Color, Rect, Vec2, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text_ex;
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};

use crate::camera::{Camera, Camera2D, TestCamera2D, Viewport};
use crate::prelude::{GameMap, SpriteContainer, Tile, TileSpriteInfo};
use crate::{Dimension2, IntVector2};

#[derive(Debug, Copy, Clone)]
pub enum RenderOp<T: Tile> {
    DrawTile(i32, i32, T),
    DrawRectangle,
    DrawCircle,
    DrawEntity(i32, i32, &'static str),
    HighlightCell(i32, i32),
    FillCell(i32, i32, Color),
}

#[derive(Debug, Copy, Clone)]
pub struct Renderer {
    pub cell_size: Dimension2,
}

impl Renderer {
    pub fn from_map_cell_size(cell_size: Dimension2) -> Self {
        Self { cell_size }
    }

    pub fn render_fov(&self, cells: Vec<IntVector2>, camera: &TestCamera2D, fov_color: Color) {
        let camera_cell_size_x = camera.cell_size.width() as f32 * camera.zoom_scale;
        let camera_cell_size_y = camera.cell_size.height() as f32 * camera.zoom_scale;

        for cell in cells {
            let (viewport_x, viewport_y) = camera.tile_to_viewport(cell).into();

            draw_rectangle(
                viewport_x,
                viewport_y,
                camera_cell_size_x,
                camera_cell_size_y,
                fov_color,
            );
        }
    }

    pub fn batch_render<T: Tile>(
        &self,
        camera: &TestCamera2D,
        viewport: &Viewport,
        sprites: &SpriteContainer,
        render_ops: &Vec<RenderOp<T>>,
    ) {
        let camera_cell_size_x = camera.cell_size.width() as f32 * camera.zoom_scale;
        let camera_cell_size_y = camera.cell_size.height() as f32 * camera.zoom_scale;
        for render_op in render_ops {
            match render_op {
                RenderOp::DrawEntity(x, y, info) => {
                    let (viewport_x, viewport_y) =
                        camera.tile_to_viewport(IntVector2::new(*x, *y)).into();
                    let (rect, texture) = sprites.get_sprite(info);

                    draw_texture_ex(
                        texture,
                        viewport_x,
                        viewport_y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(*rect),
                            dest_size: Some(Vec2::new(camera_cell_size_x, camera_cell_size_y)),
                            ..Default::default()
                        },
                    );
                }
                RenderOp::FillCell(x, y, color) => {
                    // let (viewport_x, viewport_y) = camera
                    //     .world_to_viewport(
                    //         Vec2::new(
                    //             *x as f32 * self.cell_size.width() as f32,
                    //             *y as f32 * self.cell_size.height() as f32,
                    //         ),
                    //         viewport,
                    //     )
                    //     .into();

                    let (viewport_x, viewport_y) =
                        camera.tile_to_viewport(IntVector2::new(*x, *y)).into();

                    draw_rectangle(
                        viewport_x,
                        viewport_y,
                        camera_cell_size_x,
                        camera_cell_size_y,
                        *color,
                    );
                }
                RenderOp::DrawTile(x, y, tile) => {
                    let (viewport_x, viewport_y) =
                        camera.tile_to_viewport(IntVector2::new(*x, *y)).into();

                    let sprite_info = tile.sprite_info();

                    let scaled_cell_width = self.cell_size.width() as f32 / camera.zoom_scale;
                    let scaled_cell_height = self.cell_size.height() as f32 / camera.zoom_scale;

                    match sprite_info {
                        TileSpriteInfo::None => {}
                        TileSpriteInfo::Fill(color) => {
                            draw_rectangle(
                                viewport_x,
                                viewport_y,
                                camera_cell_size_x,
                                camera_cell_size_y,
                                color,
                            );
                        }
                        TileSpriteInfo::SpriteSheet(name) => {
                            let (rect, texture) = sprites.get_sprite(name);

                            draw_texture_ex(
                                texture,
                                viewport_x,
                                viewport_y,
                                WHITE,
                                DrawTextureParams {
                                    source: Some(*rect),
                                    dest_size: Some(Vec2::new(
                                        camera_cell_size_x,
                                        camera_cell_size_y,
                                    )),
                                    ..Default::default()
                                },
                            );
                        }
                        TileSpriteInfo::SingleSprite(texture) => {
                            draw_texture_ex(
                                &texture,
                                viewport_x,
                                viewport_y,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(
                                        camera_cell_size_x,
                                        camera_cell_size_y,
                                    )),
                                    ..Default::default()
                                },
                            );
                        }
                    };

                    if let Some(items) = tile.items() {
                        draw_text_ex(
                            &format!("{}", items.len()),
                            viewport_x + scaled_cell_width / 2.0,
                            viewport_y + scaled_cell_height / 2.0,
                            Default::default(),
                        );
                    }
                    // if !tile.is_visited() {
                    //     draw_rectangle(
                    //         viewport_x,
                    //         viewport_y,
                    //         self.cell_size.width() as f32 / camera.zoom_scale,
                    //         self.cell_size.height() as f32 / camera.zoom_scale,
                    //         Color {
                    //             r: 0.0,
                    //             g: 0.0,
                    //             b: 0.0,
                    //             a: 0.8,
                    //         },
                    //     );
                    // }

                    match (tile.is_visible(), tile.is_visited()) {
                        (false, true) => {
                            draw_rectangle(
                                viewport_x,
                                viewport_y,
                                camera_cell_size_x,
                                camera_cell_size_y,
                                Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.3,
                                },
                            );
                        }
                        (false, false) => {
                            draw_rectangle(
                                viewport_x,
                                viewport_y,
                                camera_cell_size_x,
                                camera_cell_size_y,
                                Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.6,
                                },
                            );
                        }
                        (_, _) => {}
                    }

                    // draw_rectangle(
                    //     viewport_x,
                    //     viewport_y,
                    //     self.cell_size.width() as f32 * camera.zoom_scale,
                    //     self.cell_size.height() as f32 * camera.zoom_scale,
                    //     Color {
                    //         r: 1.0,
                    //         g: 0.,
                    //         b: 0.,
                    //         a: 0.6,
                    //     },
                    // );
                }
                RenderOp::DrawRectangle => {}
                RenderOp::DrawCircle => {}
                RenderOp::HighlightCell(x, y) => {
                    let (viewport_x, viewport_y) =
                        camera.tile_to_viewport(IntVector2::new(*x, *y)).into();

                    draw_rectangle(
                        viewport_x,
                        viewport_y,
                        self.cell_size.width() as f32 / camera.zoom_scale,
                        self.cell_size.height() as f32 / camera.zoom_scale,
                        Color {
                            r: 1.0,
                            g: 0.,
                            b: 0.,
                            a: 0.6,
                        },
                    );
                }
            }
        }
    }
}
