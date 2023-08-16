use bevy_ecs::{
    archetype::Archetypes,
    component::ComponentId,
    prelude::Entity,
    query::With,
    system::{Commands, Query, Res},
    world::World,
};
use macroquad::{
    prelude::{vec2, Color, Rect, RectOffset, Vec2, DARKGREEN},
    ui::{hash, root_ui, widgets, Skin},
};
use rs_nonamerl_core::prelude::Viewport;

use crate::{
    components::{CharacterInfo, Health, Inventory, Item, ModHealth, Player},
    resources::{CurrentCellInfo, GameContext, UiConfig},
};

pub fn setup_ui(world: &mut World) {
    let ui_font = include_bytes!("../../../../assets/fonts/dealerplate_california.otf");
    let skin = {
        let label_style = root_ui()
            .style_builder()
            .font(ui_font)
            .unwrap()
            .text_color(Color::from_hex(0xc9cca1))
            .font_size(20)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(20.0, -30.0, 0.0, 0.0))
            .color(Color::from_hex(0x543344))
            .build();

        Skin {
            window_style,

            label_style,
            ..root_ui().default_skin()
        }
    };

    let label_title_skin = {
        let label_style = root_ui()
            .style_builder()
            .font(ui_font)
            .unwrap()
            .text_color(Color::from_hex(0x00ffff))
            .font_size(32)
            .build();

        Skin {
            label_style,
            ..root_ui().default_skin()
        }
    };
    // commands.insert_resource(UiConfig { skin });
    world.insert_resource(UiConfig {
        skin,
        label_title_skin,
    });
}

pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().iter().any(|e| e.entity() == *entity) {
            return Some(archetype.components());
        }
    }
    None
}

pub fn draw_ui(
    ui_config: Res<UiConfig>,
    viewport: Res<Viewport>,
    query: Query<(&CharacterInfo, &Health, &Inventory), With<Player>>,
    current_cell_info: Res<CurrentCellInfo>,
    game_ctx: Res<GameContext>,
    world: &World,
    archetypes: &Archetypes,
) {
    let ui_skin = &ui_config.skin;
    let label_title_skin = &ui_config.label_title_skin;
    root_ui().push_skin(ui_skin);

    let (character_info, health, inventory) = query.single();

    widgets::Window::new(
        hash!(),
        vec2(viewport.x + viewport.width + 100., viewport.y),
        vec2(720., 800.),
    )
    // .label("Camera")
    // .titlebar(true)
    .movable(false)
    .ui(&mut root_ui(), |ui| {
        ui.push_skin(label_title_skin);
        ui.label(None, "Player UI");
        ui.pop_skin();

        ui.label(None, &format!("Health: {}/{}", health.current, health.max));
        ui.label(None, &format!("Strength: {}", character_info.strength));
        ui.label(None, &format!("Stamina: {}", character_info.stamina));
        ui.label(
            None,
            &format!("Intelligence: {}", character_info.intelligence),
        );
        ui.label(None, &format!("Dexterity: {}", character_info.dexterity));
        ui.label(
            None,
            &format!(
                "XP: {}/{}",
                character_info.xp.current, character_info.xp.max
            ),
        );
        ui.push_skin(label_title_skin);
        ui.label(None, "Equipment");
        ui.pop_skin();
        ui.label(
            None,
            &format!(
                "Gold: {}/{}",
                character_info.gold.current, character_info.gold.total
            ),
        );
        ui.push_skin(label_title_skin);
        ui.label(None, "Actions");
        ui.pop_skin();
        ui.label(None, "left arrow: move left");
        ui.label(None, "right arrow: move right");
        ui.label(None, "up arrow: move up");
        ui.label(None, "down arrow: move down");
        ui.push_skin(label_title_skin);
        ui.label(
            None,
            &format!("Interactions [{}:?]", current_cell_info.len()),
        );
        ui.pop_skin();
        current_cell_info
            .interactions()
            .iter()
            .for_each(|interaction| {
                ui.label(None, &format!("{}: {}", interaction.key, interaction.kind));
            });
        ui.push_skin(label_title_skin);
        ui.label(None, "Game Context");
        ui.pop_skin();

        ui.label(None, &format!("Current: {:?}", game_ctx.state));
        ui.label(None, &format!("Inventory: {:?}", inventory.items));

        ui.push_skin(label_title_skin);
        ui.label(None, "Current Cell Items");
        ui.pop_skin();

        if let Some(tile) = current_cell_info.current_tile() {
            let tile_items = tile.items.clone();
            tile_items.iter().for_each(|item| {
                let item_data = world.get::<Item>(*item).unwrap();
                ui.label(None, &format!("{:?}: {:?}", item_data.kind, item_data.name));
                if let Some(mod_health) = world.get::<ModHealth>(*item) {
                    ui.label(None, &format!("Inventory: {:?}", mod_health));
                }
            });
        }
    });
}
