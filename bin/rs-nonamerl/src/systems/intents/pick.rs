use bevy_ecs::{
    entity,
    prelude::{Entity, EventWriter},
    system::{Command, Commands, Query, Res},
    world::World,
};
use rs_nonamerl_core::prelude::GameMap;

use tracing::instrument;

use crate::{
    components::{Inventory, PickIntent, Player, Position},
    events::UpdateAvailableInteractionsEvent,
    tiles::TestTile,
};

#[derive(Debug, Clone)]
pub struct PickAction {
    pub entity: Entity,
    pub tile_position: Position,
    pub item: Entity,
}

impl Command for PickAction {
    fn apply(self, world: &mut World) {
        let mut game_map = world.get_resource_mut::<GameMap<TestTile>>().unwrap();
        let mut tile = game_map
            .get(self.tile_position.x, self.tile_position.y)
            .unwrap();
        tile.items.retain(|item| *item != self.item);
        game_map.set(self.tile_position.x, self.tile_position.y, tile);
        let (player, mut inventory) = world
            .query::<(Entity, &mut Inventory)>()
            .get_single_mut(world)
            .unwrap();

        inventory.items.push(self.item);
    }
}

#[instrument(skip(commands, game_map, intents, writer))]
pub fn pick_intent_system(
    intents: Query<(Entity, &Position, &PickIntent)>,
    mut commands: Commands,
    mut writer: EventWriter<UpdateAvailableInteractionsEvent>,
    game_map: Res<GameMap<TestTile>>,
    // world: &World,
) {
    for (entity, position, intent) in intents.iter() {
        tracing::debug!("Picking intent {:?} for entity {:?}", intent.item, entity);

        // if an item is specified, pick it up
        if let Some(item) = intent.item {
            commands.add(PickAction {
                entity,
                tile_position: position.clone(),
                item,
            });

            let tile = game_map.get(position.x, position.y).unwrap();
            let items = tile.items.clone();

            writer.send(UpdateAvailableInteractionsEvent {
                position: position.clone(),
            });
        }

        commands.entity(entity).remove::<PickIntent>();
    }
}
