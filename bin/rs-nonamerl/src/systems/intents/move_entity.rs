use bevy_ecs::{
    prelude::{Entity, EventWriter},
    query::{Changed, With},
    system::{Command, Commands, Query, Res},
    world::World,
};
use rs_nonamerl_core::{prelude::GameMap, IntVector2};

use crate::{
    components::{MoveIntent, Player, Position},
    events::UpdateAvailableInteractionsEvent,
    tiles::TestTile,
    Walkable,
};

use tracing::instrument;

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

#[instrument(skip(query, writer))]
pub fn on_player_moved_system(
    query: Query<&Position, (Changed<Position>, With<Player>)>,
    // game_map: Res<GameMap<TestTile>>,
    // mut commands: Commands,
    mut writer: EventWriter<UpdateAvailableInteractionsEvent>,
    // available_interactions: ResMut<AvaliableInteractions>,
    // world: &World,
) {
    if query.iter().count() == 0 {
        return;
    }
    let position = query.single();
    // let tile = game_map.get(position.x, position.y).unwrap();
    // let items = tile.items.clone();

    writer.send(UpdateAvailableInteractionsEvent {
        position: position.clone(),
    });
    // let mut current_available_interactions = Vec::<Interaction>::new();

    // items.iter().for_each(|item| {
    //     let item_entity = *item;

    //     tracing::info!("Item {:?} is on current cell", item_entity);
    //     // let item = world.get::<
    //     if let Some(interactions) = world.get::<Interactions>(item_entity) {
    //         tracing::info!(
    //             "Item {:?} has interactions : {:?}",
    //             item_entity,
    //             interactions
    //         );
    //         current_available_interactions.extend(interactions.interactions.clone());
    //     }
    // });
    // commands.add(commands::UpdateAvailableInteractions {
    //     interactions: current_available_interactions,
    //     current_tile: tile,
    // });
}

#[instrument(skip(commands, game_map, world, intents))]
pub fn move_intent_system(
    intents: Query<(Entity, &Position, &MoveIntent)>,
    mut commands: Commands,
    game_map: Res<GameMap<TestTile>>,
    world: &World,
) {
    // let _span = tracy_client::span!("move_intent_system");
    tracy_client::Client::running()
        .expect("client must be running")
        .plot(
            tracy_client::plot_name!("MoveIntentSystem::entities"),
            intents.iter().count() as f64,
        );
    for (entity, position, intent) in intents.iter() {
        tracy_client::Client::running().unwrap().message(
            &format!(
                "Entity {:?} wants to move from {:?} to {:?}",
                entity, position, intent.target
            ),
            0,
        );
        tracing::debug!(
            "Entity {:?} wants to move from {:?} to {:?}",
            entity,
            position,
            intent.target
        );

        // get current tile
        // let tile = game_map.get(position.x, position.y).unwrap();

        let is_player = world
            .get::<Player>(entity)
            .map(|_| true)
            .unwrap_or_else(|| false);

        // get target tile
        let target_tile = game_map.get(intent.target.x, intent.target.y);

        if target_tile.is_none() {
            continue;
            //TODO: handle this
        }

        // check if target tile is walkable
        if target_tile.unwrap().is_walkable() {
            commands.add(MoveAction {
                source: IntVector2::new(position.x, position.y),
                target: intent.target,
                entity,
            });
            if is_player {
                // add interaction intents keybindings to ui
                // commands.add(UpdateIntents {
                //     source: IntVector2::new(position.x, position.y),
                // });
                // let tile_items = target_tile.unwrap().items.clone();
                // tile_items.iter().for_each(|item| {
                //     let item_entity = *item;
                //     // let item = world.get::<Uses>(item_entity).unwrap();
                //     // commands.entity(item_entity).insert();
                // });
                // commands.entity(entity).insert(...);
            }

            tracing::debug!(
                "Actions: entity {:?} move from {:?} to {:?}",
                entity,
                position,
                intent.target
            );
        }

        // remove move intent
        commands.entity(entity).remove::<MoveIntent>();
    }
}
