use bevy_ecs::system::Command;
use rs_nonamerl_core::prelude::GameMap;

use crate::{
    components::{Interaction, Interactions, Position},
    resources::CurrentCellInfo,
    tiles::TestTile,
};

#[derive(Debug, Clone)]
pub struct UpdateAvailableInteractions {
    pub position: Position,
}

impl Command for UpdateAvailableInteractions {
    fn apply(self, world: &mut bevy_ecs::world::World) {
        let game_map = world.get_resource::<GameMap<TestTile>>().unwrap();
        let tile = game_map.get(self.position.x, self.position.y).unwrap();
        let items = tile.items.clone();

        // let items = event.items.clone();
        // let tile = event.tile.clone();
        let mut current_available_interactions = Vec::<Interaction>::new();

        items.iter().for_each(|item| {
            let item_entity = *item;

            tracing::info!("Item {:?} is on current cell", item_entity);
            // let item = world.get::<
            if let Some(interactions) = world.get::<Interactions>(item_entity) {
                tracing::info!(
                    "Item {:?} has interactions : {:?}",
                    item_entity,
                    interactions
                );
                current_available_interactions.extend(interactions.interactions.clone());
            }
        });
        let mut available_interactions = world.get_resource_mut::<CurrentCellInfo>().unwrap();
        available_interactions.clear();
        tracing::debug!(
            "Updating available interactions: {:?}",
            available_interactions.len()
        );
        available_interactions.add_all(current_available_interactions);
        available_interactions.set_current_tile(tile.clone());
        tracing::debug!(
            "Available interactions: {:?} | current tile = {:?}",
            available_interactions.len(),
            available_interactions.current_tile()
        );
    }
}
