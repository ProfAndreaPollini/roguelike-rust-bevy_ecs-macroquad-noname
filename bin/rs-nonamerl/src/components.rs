#![allow(dead_code)]
use std::fmt::Display;

use bevy_ecs::{
    prelude::{Component, Entity},
    system::{Command, Commands},
    world::World,
};
use rs_nonamerl_core::{prelude::KeyInput, IntVector2};

use crate::tiles::TestTile;

#[derive(Component, Default, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Player {}

#[derive(Component, Default, Debug, Clone)]
pub struct Enemy {}

#[derive(Component, Default, Debug, Clone)]
pub struct CharacterInfo {
    pub strength: i32,
    pub stamina: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub name: String,
    pub xp: Xp,
    pub gold: Gold,
}

#[derive(Default, Debug, Clone)]
pub struct Gold {
    pub current: i32,
    pub total: i32,
}

#[derive(Default, Debug, Clone)]
pub struct Xp {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    None,
    Gold,
    Supplies,
    Potion,
    Wand,
    Weapon,
    Armor,
    Light,
    Ammo,
    Shield,
    Scrolls,
    SpellBook,
}

impl Default for ItemKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub kind: ItemKind,
}

#[derive(Default, Debug, Clone)]
pub struct ItemUse {
    key: KeyInput,
}

// #[derive(Default, Debug, Clone)]
// pub struct ItemUses {
//     pub uses: Vec<ItemUse>,
// }

#[derive(Component, Default, Debug, Clone)]
pub struct ModHealth {
    pub amount: i32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Entity>,
    pub capacity: usize,
}

#[derive(Component, Default, Debug, Clone)]
pub struct SpriteDrawInfo {
    pub sprite_info: &'static str,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct MoveIntent {
    pub target: IntVector2,
}

// pub enum Effect {
//     None,
// }

#[derive(Component, Default, Debug, Clone)]
pub struct PickIntent {
    pub item: Option<Entity>,
    pub tile: Option<TestTile>,
}

impl PickIntent {
    pub fn new() -> Self {
        Self {
            item: None,
            tile: None,
        }
    }

    pub fn from(tile: Option<TestTile>) -> Self {
        if tile.is_none() {
            return Self::new();
        }
        let item = tile.clone().unwrap().items.first().cloned();
        Self { item, tile }
    }
}

#[derive(Debug, Clone)]
pub enum UseKind {
    None,
    Pick,
}

impl Display for UseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseKind::None => write!(f, "None"),
            UseKind::Pick => write!(f, "Pick"),
        }
    }
}

impl Default for UseKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct Interaction {
    pub key: KeyInput,
    pub kind: UseKind,
}

impl Command for Interaction {
    fn apply(self, world: &mut World) {
        tracing::info!("Interaction::apply [ {:?} ]", self.kind);
    }
}

impl Interaction {
    pub fn new(key: KeyInput, kind: UseKind) -> Self {
        Self { key, kind }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct Interactions {
    pub interactions: Vec<Interaction>,
}

impl Interactions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: KeyInput, kind: UseKind) {
        self.interactions.push(Interaction { key, kind });
    }
}

// #[derive(Component, Debug, Clone)]
// pub struct PickItem {
//     pub key: KeyInput,
//     pub item_id: Option<Entity>,
// }

// impl PickItem {
//     pub fn new(key: KeyInput) -> Self {
//         Self { key, item_id: None }
//     }

//     pub fn use_item(&self, commands: &mut Commands, entity: Entity) {
//         commands.entity(entity).insert(MoveIntent {
//             target: IntVector2::new(0, 0),
//         });
//     }
// }

// impl Command for PickItem {
//     fn apply(self, world: &mut World) {
//         tracing::info!("PickItem::apply");
//     }
// }

#[derive(Debug, Clone)]
pub struct MoveAction {
    pub entity: Entity,
    pub source: IntVector2,
    pub target: IntVector2,
}

impl Command for MoveAction {
    fn apply(self, world: &mut World) {
        let mut position = world.get_mut::<Position>(self.entity).unwrap();
        position.x = self.target.x;
        position.y = self.target.y;
    }
}

#[derive(Debug, Clone)]
pub struct UseItemAction {
    pub entity: Entity,
    pub item: Entity,
}

#[derive(Debug, Clone)]
pub struct TestCommand {}

impl Command for TestCommand {
    fn apply(self, _world: &mut World) {
        tracing::info!("TestCommand::apply");
    }
}
