use bevy_ecs::{prelude::Entity, system::Resource};

use crate::{prelude::Tile, IntVector2};

use super::GameMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapCommand {
    SetVisited(IntVector2, bool),
    SetVisible(IntVector2, bool),
    AddItem(IntVector2, Entity),
    RemoveItem(IntVector2, Entity),
}

#[derive(Debug, Clone, Resource)]
pub struct MapCommands {
    pub commands: Vec<MapCommand>,
}

impl MapCommands {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: MapCommand) {
        self.commands.push(command);
    }

    pub fn add_all(&mut self, commands: Vec<MapCommand>) {
        self.commands.extend(commands);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn process_commands<T: Tile>(&mut self, map: &mut GameMap<T>) {
        for command in self.commands.iter() {
            match command {
                MapCommand::SetVisited(pos, visited) => {
                    map.set_visited(*pos, *visited);
                }
                MapCommand::SetVisible(pos, visible) => {
                    map.set_visible(*pos, *visible);
                }
                MapCommand::AddItem(pos, item) => {
                    map.add_item(*pos, *item);
                }
                MapCommand::RemoveItem(pos, item) => {
                    map.remove_item(*pos, *item);
                }
            }
        }
        self.commands.clear();
    }
}

impl Default for MapCommands {
    fn default() -> Self {
        Self::new()
    }
}
