use bevy_ecs::system::Resource;
use macroquad::prelude::Vec2;

use crate::{Dimension2, IntExtent2, IntVector2};

#[derive(Debug, Copy, Clone, Default, Resource)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    // TODO: remove this
    pub fn contains_screen_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.contains_screen_point(point.x, point.y)
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

pub trait Camera {
    fn from_viewport(
        position: Vec2,
        viewport: &Viewport,
        zoom_scale: f32,
        cell_size: Dimension2,
    ) -> Self;
    fn world_to_viewport(&self, world_pos: Vec2, viewport: &Viewport) -> Vec2;
    fn viewport_to_world(&self, viewport_pos: Vec2, viewport: &Viewport) -> Vec2;
    fn center_on_world_point(&mut self, target_pos: Vec2, viewport: &Viewport);
    fn get_visibile_extent(&self, viewport: &Viewport, cell_w: usize, cell_h: usize) -> IntExtent2;
    fn cell_of(&self, world_pos: Vec2) -> IntVector2;
    fn move_right(&mut self, amount: f32);
}

#[derive(Debug, Copy, Clone, Default, Resource)]
pub struct Camera2D {
    pub position: Vec2,
    pub fov: Vec2,
    pub zoom_scale: f32,
    pub cell_size: Dimension2,
}

impl Camera for Camera2D {
    // Costruttore: inizializza il FOV basandosi sulle dimensioni del viewport
    fn from_viewport(
        position: Vec2,
        viewport: &Viewport,
        zoom_scale: f32,
        cell_size: Dimension2,
    ) -> Self {
        let aspect_ratio = viewport.width / viewport.height;
        let fov_width = viewport.width;
        let fov_height = viewport.width / aspect_ratio;

        Camera2D {
            position,
            fov: Vec2::new(fov_width, fov_height),
            zoom_scale,
            cell_size,
        }
    }

    fn move_right(&mut self, amount: f32) {
        self.position.x += amount as f32 * self.cell_size.width() as f32 * self.zoom_scale as f32;
    }

    // Metodo per posizionare la camera in modo tale da avere il punto specifico al centro del FOV
    fn center_on_world_point(&mut self, target: Vec2, viewport: &Viewport) {
        let (target_x, target_y) = target.into();
        let (fov_width, fov_height) = self.fov.into();
        //let camera_relative = target - (self.fov * self.zoom_scale) / 2.0;
        let camera_relative_x = target_x - (fov_width * self.zoom_scale) / 2.0;
        let camera_relative_y = target_y - (fov_height * self.zoom_scale) / 2.0;
        self.position.x = camera_relative_x;
        self.position.y = camera_relative_y;
    }

    /// maps a point from viewport space to world space
    fn viewport_to_world(&self, viewport_pos: Vec2, viewport: &Viewport) -> Vec2 {
        let (viewport_x, viewport_y) = viewport_pos.into();
        let (fov_width, fov_height) = self.fov.into();
        let (position_x, position_y) = self.position.into();

        // Calcoliamo le coordinate normalizzate rispetto alle dimensioni del viewport
        let normalized_x = (viewport_x - viewport.x) / viewport.width;
        let normalized_y = (viewport_y - viewport.y) / viewport.height;

        // Calcoliamo le coordinate relative alla camera
        let camera_relative_x = normalized_x * fov_width * self.zoom_scale;
        let camera_relative_y = normalized_y * fov_height * self.zoom_scale;

        // Calcoliamo le coordinate assolute nel mondo di gioco
        let world_x = position_x + camera_relative_x;
        let world_y = position_y + camera_relative_y;

        Vec2::new(world_x, world_y)
        //  (world_x, world_y)
    }

    /// maps a point from world space to viewport space
    fn world_to_viewport(&self, world_pos: Vec2, viewport: &Viewport) -> Vec2 {
        // // let camera_relative_x = world_x - self.position_x;
        // // let camera_relative_y = world_y - self.position_y;
        // let camera_relative = world_pos - self.position;

        // let normalized_x = camera_relative.x / (self.fov.x * self.zoom_scale);
        // let normalized_y = camera_relative.y / (self.fov.y * self.zoom_scale);

        // let viewport_x_pos = viewport.x + normalized_x * viewport.width;
        // let viewport_y_pos = viewport.y + normalized_y * viewport.height;

        // Vec2::new(viewport_x_pos, viewport_y_pos)
        // //(viewport_x_pos, viewport_y_pos)
        let (world_x, world_y) = world_pos.into();
        let (fov_width, fov_height) = self.fov.into();
        let (position_x, position_y) = self.position.into();

        let camera_relative_x = world_x - position_x;
        let camera_relative_y = world_y - position_y;

        let normalized_x = camera_relative_x / (fov_width * self.zoom_scale);
        let normalized_y = camera_relative_y / (fov_height * self.zoom_scale);

        let viewport_x_pos = viewport.x + normalized_x * viewport.width;
        let viewport_y_pos = viewport.y + normalized_y * viewport.height;

        Vec2::new(viewport_x_pos, viewport_y_pos)
    }

    fn get_visibile_extent(&self, viewport: &Viewport, cell_w: usize, cell_h: usize) -> IntExtent2 {
        // dbg!(self.position, self.fov, viewport);
        let min_cell = self.viewport_to_world(Vec2::new(viewport.x, viewport.y), viewport);

        let max_cell = self.viewport_to_world(
            Vec2::new(viewport.x + viewport.width, viewport.y + viewport.height),
            viewport,
        );

        let min_cell = (
            (min_cell.x / cell_w as f32) as i32,
            (min_cell.y / cell_h as f32) as i32,
        );

        let max_cell = (
            (max_cell.x / cell_w as f32) as i32,
            (max_cell.y / cell_h as f32) as i32,
        );
        IntExtent2::new(
            min_cell.0,
            min_cell.1,
            max_cell.0.abs_diff(min_cell.0),
            max_cell.1.abs_diff(min_cell.1),
        )
    }

    fn cell_of(&self, world_pos: Vec2) -> IntVector2 {
        let cell_x = (world_pos.x / self.cell_size.width() as f32) as i32;
        let cell_y = (world_pos.y / self.cell_size.height() as f32) as i32;

        IntVector2::new(cell_x, cell_y)
    }
}

#[derive(Debug, Copy, Clone, Default, Resource)]
pub struct TestCamera2D {
    pub position: Vec2,
    pub viewport: Viewport,
    pub zoom_scale: f32,
    pub cell_size: Dimension2,
    pub visible_extent: IntExtent2,
    pub visible_tiles_extent: IntExtent2,
}

impl TestCamera2D {
    pub fn from_viewport(
        position: Vec2,
        viewport: &Viewport,
        zoom_scale: f32,
        cell_size: Dimension2,
    ) -> Self {
        let mut camera = TestCamera2D {
            position,
            viewport: *viewport,
            zoom_scale,
            cell_size,
            visible_extent: Default::default(),
            visible_tiles_extent: Default::default(),
        };
        camera.update();
        camera
    }

    pub fn update(&mut self) {
        self.update_visible_extent();
        self.update_visibile_tiles_extent();
    }

    fn update_visible_extent(&mut self) {
        let visible_width = self.viewport.width / self.zoom_scale;
        let visible_height = self.viewport.height / self.zoom_scale;

        let visible_x = self.position.x - visible_width / 2.0;
        let visible_y = self.position.y - visible_height / 2.0;
        self.visible_extent = IntExtent2::new(
            visible_x as i32,
            visible_y as i32,
            visible_width as u32,
            visible_height as u32,
        )
    }

    fn update_visibile_tiles_extent(&mut self) {
        /*
                visibleWidth = viewportWidth / (zoom * cell_x)
        visibleHeight = viewportHeight / (zoom * cell_y)
        visibleX = cameraX - visibleWidth / 2
        visibleY = cameraY - visibleHeight / 2
        */
        let visible_width = self.viewport.width / (self.zoom_scale * self.cell_size.width() as f32);
        let visible_height =
            self.viewport.height / (self.zoom_scale * self.cell_size.height() as f32);

        let visible_x = self.position.x - visible_width / 2.0;
        let visible_y = self.position.y - visible_height / 2.0;
        self.visible_tiles_extent = IntExtent2::new(
            visible_x as i32,
            visible_y as i32,
            visible_width as u32,
            visible_height as u32,
        )
    }

    pub fn tile_to_viewport(&self, tile_pos: IntVector2) -> Vec2 {
        /*
         projX = (x - visibleX + offsetX) * zoom * cell_x + viewportX
        projY = (y - visibleY + offsetY) * zoom * cell_y + viewportY
        */
        let visible_tiles_extent = self.visible_tiles_extent;
        let proj_x = (tile_pos.x - visible_tiles_extent.left()) as f32
            * self.zoom_scale
            * self.cell_size.width() as f32
            + self.viewport.x;
        let proj_y = (tile_pos.y - visible_tiles_extent.top()) as f32
            * self.zoom_scale
            * self.cell_size.height() as f32
            + self.viewport.y;

        Vec2::new(proj_x, proj_y)
    }

    pub fn world_to_viewport(&self, world_pos: Vec2) -> Vec2 {
        let visible_extent = self.visible_extent;
        let proj_x =
            (world_pos.x - visible_extent.left() as f32) * self.zoom_scale + self.viewport.x;
        let proj_y =
            (world_pos.y - visible_extent.top() as f32) * self.zoom_scale + self.viewport.y;

        Vec2::new(proj_x, proj_y)
    }

    pub fn viewport_to_world(&self, viewport_pos: Vec2) -> Vec2 {
        /*
        worldX = (projX - viewportX) / (zoom * cell_x) + visibleX
        worldY = (projY - viewportY) / (zoom * cell_y) + visibleY
        */
        let visible_extent = self.visible_extent;
        let proj_x =
            (viewport_pos.x - self.viewport.x) / self.zoom_scale + visible_extent.left() as f32;
        let proj_y =
            (viewport_pos.y - self.viewport.y) / self.zoom_scale + visible_extent.top() as f32;

        Vec2::new(proj_x, proj_y)
    }

    pub fn viewport_to_tile(&self, viewport_pos: Vec2) -> IntVector2 {
        /*
        x = (projX - viewportX) / (zoom * cell_x) + visibleX
        y = (projY - viewportY) / (zoom * cell_y) + visibleY
        */
        // let visible_tiles_extent = self.visible_tiles_extent;
        // // println!("visible_tiles_extent: {:?}", visible_tiles_extent);
        // let proj_x = (viewport_pos.x - self.visible_extent.left() as f32)
        //     / (self.zoom_scale * self.cell_size.width() as f32)
        //     + visible_tiles_extent.left() as f32;
        // let proj_y = (viewport_pos.y - self.visible_extent.top() as f32)
        //     / (self.zoom_scale * self.cell_size.height() as f32)
        //     + visible_tiles_extent.top() as f32;

        let world_pos = self.viewport_to_world(viewport_pos);

        IntVector2::new(
            (world_pos.x / self.cell_size.width() as f32) as i32,
            (world_pos.y / self.cell_size.height() as f32) as i32,
        )
    }
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_camera_from_viewport() {
        use super::*;

        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        };

        let camera =
            Camera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(16, 16));

        assert_eq!(camera.fov.x, 800.0);
        assert_eq!(camera.fov.y, 600.0);
    }

    #[test]
    fn test_center_on_world_point() {
        use super::*;

        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        };

        let mut camera =
            Camera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(16, 16));

        camera.center_on_world_point(Vec2::new(100.0, 100.0), &viewport);

        assert_eq!(camera.position.x, -300.0);
        assert_eq!(camera.position.y, -200.0);

        camera.center_on_world_point(Vec2::new(0.0, 0.0), &viewport);

        assert_eq!(camera.position.x, -400.0);
        assert_eq!(camera.position.y, -300.0);
    }

    #[test]
    fn test_testcamera2d() {
        use super::*;

        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 30.0,
            height: 30.0,
        };

        let mut camera =
            TestCamera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(10, 10));

        let extent = camera.visible_tiles_extent;
        assert_eq!(extent.left(), -1);
        assert_eq!(extent.top(), -1);
        assert_eq!(extent.right(), 2);
        assert_eq!(extent.bottom(), 2);
        assert_eq!(camera.position.x, 0.0);
        assert_eq!(camera.position.y, 0.0);
        let tile_pos = IntVector2::new(0, 0);
        let viewport_pos = camera.tile_to_viewport(tile_pos);
        assert_eq!(viewport_pos.x, 10.0);
        assert_eq!(viewport_pos.y, 10.0);

        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 60.0,
            height: 30.0,
        };

        let mut camera =
            TestCamera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(10, 10));

        let extent = camera.visible_tiles_extent;
        assert_eq!(extent.left(), -3);
        assert_eq!(extent.top(), -1);
        assert_eq!(extent.right(), 3);
        assert_eq!(extent.bottom(), 2);
        assert_eq!(camera.position.x, 0.0);
        assert_eq!(camera.position.y, 0.0);
        let tile_pos = IntVector2::new(0, 0);
        let viewport_pos = camera.tile_to_viewport(tile_pos);
        assert_eq!(viewport_pos.x, 30.0);
        assert_eq!(viewport_pos.y, 10.0);
    }

    #[test]
    fn test_testcamera2d_world_to_viewport() {
        use super::*;
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 30.0,
            height: 30.0,
        };

        let mut camera =
            TestCamera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(10, 10));

        let world_pos = Vec2::new(0.0, 0.0);

        let viewport_pos = camera.world_to_viewport(world_pos);

        assert_eq!(viewport_pos.x, 15.0);
        assert_eq!(viewport_pos.y, 15.0);
    }

    #[test]
    fn test_testcamera2d_world_to_viewport_camera_update() {
        use super::*;
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 30.0,
            height: 30.0,
        };

        let mut camera =
            TestCamera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(10, 10));

        let world_pos = Vec2::new(0.0, 0.0);
        camera.position = Vec2::new(10.0, 10.0);
        camera.update();
        let viewport_pos = camera.world_to_viewport(world_pos);

        assert_eq!(viewport_pos.x, 5.0);
        assert_eq!(viewport_pos.y, 5.0);
        assert_eq!(camera.visible_extent.left(), -5);
        assert_eq!(camera.visible_extent.top(), -5);
        assert_eq!(camera.visible_extent.right(), 25);
        assert_eq!(camera.visible_extent.bottom(), 25);
    }

    #[test]

    fn test_testcamera2d_viewport_to_world() {
        use super::*;
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: 30.0,
            height: 30.0,
        };

        let mut camera =
            TestCamera2D::from_viewport(Vec2::default(), &viewport, 1.0, Dimension2::new(10, 10));

        let viewport_pos = Vec2::new(15.0, 15.0);

        let world_pos = camera.viewport_to_world(viewport_pos);

        assert_eq!(world_pos.x, 0.0);
        assert_eq!(world_pos.y, 0.0);

        let tile_pos = camera.viewport_to_tile(viewport_pos);

        assert_eq!(tile_pos.x, 0);
        assert_eq!(tile_pos.y, 0);

        camera.position = Vec2::new(10.0, 10.0);
        camera.update();
        let viewport_pos = camera.world_to_viewport(world_pos);

        assert_eq!(viewport_pos.x, 5.0);
        assert_eq!(viewport_pos.y, 5.0);
        assert_eq!(camera.visible_extent.left(), -5);
        assert_eq!(camera.visible_extent.top(), -5);
        assert_eq!(camera.visible_extent.right(), 25);
        assert_eq!(camera.visible_extent.bottom(), 25);

        let tile_pos = camera.viewport_to_tile(Vec2::default());

        assert_eq!(tile_pos.x, 2);
        assert_eq!(tile_pos.y, 2);
    }
}
