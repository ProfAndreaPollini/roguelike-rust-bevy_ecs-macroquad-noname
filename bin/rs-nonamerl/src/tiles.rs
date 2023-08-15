use bevy_ecs::prelude::Entity;
use rs_nonamerl_core::prelude::{
    FovOccluder, ItemContainer, Tile, TileSpriteInfo, VisibilityOcclusion, Visible, Visited,
    Walkable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileKind {
    Grass,
    Floor,
    Wall(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestTile {
    pub kind: TileKind,
    pub visited: bool,
    pub visible: bool,
    pub items: Vec<Entity>,
}

impl TestTile {
    pub fn new(kind: TileKind) -> Self {
        Self {
            kind,
            visited: false,
            visible: false,
            items: Vec::new(),
        }
    }
}

impl Default for TestTile {
    fn default() -> Self {
        Self {
            kind: TileKind::Grass,
            visited: false,
            visible: false,
            items: Vec::new(),
        }
    }
}

impl Tile for TestTile {
    fn sprite_info(&self) -> TileSpriteInfo {
        match self.kind {
            TileKind::Grass => TileSpriteInfo::SpriteSheet("grass"),
            TileKind::Floor => TileSpriteInfo::SpriteSheet("floor"),
            TileKind::Wall(wall_name) => TileSpriteInfo::SpriteSheet(wall_name),
        }
    }
}
impl Visible for TestTile {
    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
impl Visited for TestTile {
    fn is_visited(&self) -> bool {
        self.visited
    }

    fn set_visited(&mut self, visited: bool) {
        self.visited = visited;
    }
}
impl FovOccluder for TestTile {
    fn block_visibility(&self) -> VisibilityOcclusion {
        match self.kind {
            TileKind::Wall(_) => TestTile::BLOCKED,
            _ => TestTile::VISIBLE,
        }
    }
}
impl Walkable for TestTile {
    fn is_walkable(&self) -> bool {
        !matches!(self.kind, TileKind::Wall(_))
    }
}

impl ItemContainer for TestTile {
    fn items(&self) -> Option<Vec<Entity>> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.clone())
        }
    }

    fn add_item(&mut self, item: Entity) {
        println!("add_item: {:?}", item);
        self.items.push(item);
    }

    fn remove_item(&mut self, item: Entity) {
        self.items.retain(|i| *i != item);
    }
}
