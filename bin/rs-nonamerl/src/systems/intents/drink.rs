use bevy_ecs::{
    prelude::{Entity, EventWriter},
    query::With,
    system::{Command, Commands, Query, Res},
    world::World,
};
use rs_nonamerl_core::prelude::GameMap;

use tracing::instrument;

use crate::remove_item_from_cell;
use crate::{
    components::{DrinkEffect, DrinkIntent, Health, Inventory, PickIntent, Player, Position},
    events::UpdateAvailableInteractionsEvent,
    tiles::TestTile,
};

// #[derive(Debug, Clone)]
// pub struct PickAction {
//     pub entity: Entity,
//     pub tile_position: Position,
//     pub item: Entity,
// }

// impl Command for PickAction {
//     fn apply(self, world: &mut World) {
//         let game_map = world.get_resource_mut::<GameMap<TestTile>>().unwrap();
//         let mut tile = game_map
//             .get(self.tile_position.x, self.tile_position.y)
//             .unwrap();
//         tile.items.retain(|item| *item != self.item);
//         game_map.set(self.tile_position.x, self.tile_position.y, tile);
//         let (mut inventory) = world
//             .query::<(&mut Inventory, With<Player>)>()
//             .get_single_mut(world)
//             .unwrap();

//         //TODO: remove position from item

//         inventory.0.items.push(self.item);
//     }
// }

#[derive(Debug, Clone)]
pub struct DrinkAction {
    pub entity: Entity,
    pub effect: DrinkEffect,
    pub item: Entity,
    pub position: Position,
}

impl Command for DrinkAction {
    fn apply(self, world: &mut World) {
        tracing::info!("DrinkAction::apply");
        if self.effect.health != 0 {
            if let Some(mut health) = world.get_mut::<Health>(self.entity) {
                health.current += self.effect.health;
            }
        }
        remove_item_from_cell(world, &self.position, self.item);
    }
}

#[instrument(skip(commands, game_map, intents, writer))]
pub fn drink_intent_system(
    intents: Query<(Entity, &Position, &DrinkIntent)>,
    mut commands: Commands,
    mut writer: EventWriter<UpdateAvailableInteractionsEvent>,
    game_map: Res<GameMap<TestTile>>,
    // world: &World,
) {
    for (entity, position, intent) in intents.iter() {
        tracing::debug!(
            "entity {:?} wants to drink item {:?}  ",
            entity,
            intent.item
        );

        // if an item is specified, pick it up
        if let Some(item) = intent.item {
            commands.add(DrinkAction {
                entity,
                effect: intent.effect.clone().unwrap(),
                item,
                position: position.clone(),
            });

            let tile = game_map.get(position.x, position.y).unwrap();
            // let tile_items = tile.items.clone();

            writer.send(UpdateAvailableInteractionsEvent {
                position: position.clone(),
            });
        }

        commands.entity(entity).remove::<DrinkIntent>();
    }
}
