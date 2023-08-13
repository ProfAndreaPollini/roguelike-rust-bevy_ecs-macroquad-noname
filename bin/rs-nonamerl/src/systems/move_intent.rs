use bevy_ecs::{
    prelude::Entity,
    system::{Commands, Query, Res},
    world::World,
};
use rs_nonamerl_core::{prelude::GameMap, IntVector2};

use crate::{
    components::{Effect, MoveAction, MoveIntent, Player, Position},
    tiles::TestTile,
    Walkable,
};

use tracing::instrument;

#[instrument(skip(commands, game_map, world))]
pub fn move_action_system(commands: Commands, game_map: Res<GameMap<TestTile>>, world: &World) {
    // tracing::info!(
    //     "move_action_system [{}]",
    //     actions.actions.lock().unwrap().len()
    // );
    // if let Some(action) = actions.actions.lock().unwrap().pop() {
    //     tracing::info!("Executing action");
    //     let effect = action.execute(game_map, world.get_mut(entity));
    //     if let Some(effect) = effect {
    //         match effect {
    //             Effect::None => {}
    //         }
    //     }
    // }
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
        tracing::info!(
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
            tracing::info!(
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
