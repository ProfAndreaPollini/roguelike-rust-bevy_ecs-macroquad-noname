use bevy_ecs::{
    prelude::EventReader,
    system::{Commands, ResMut},
    world::{self, World},
};

use crate::{
    commands,
    components::{Interaction, Interactions},
    events::{ChangeGameStateEvent, UpdateAvailableInteractionsEvent},
    resources::GameContext,
};

pub fn change_game_state(
    mut reader: EventReader<ChangeGameStateEvent>,
    mut game_ctx: ResMut<GameContext>,
) {
    if reader.is_empty() {
        return;
    }
    for event in reader.iter() {
        tracing::info!(
            "change_game_state system  {:?}-> {:?}",
            game_ctx.state,
            event
        );
        game_ctx.state = event.new_state.clone();
    }
}

pub fn update_available_interactions(
    mut reader: EventReader<UpdateAvailableInteractionsEvent>,

    mut commands: Commands,
) {
    if reader.is_empty() {
        return;
    }
    for event in reader.iter() {
        let position = event.position.clone();
        // let tile = game_map.get(position.x, position.y).unwrap();
        // let items = tile.items.clone();

        // let items = event.items.clone();
        // let tile = event.tile.clone();
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
        commands.add(commands::UpdateAvailableInteractions { position });
    }
}
