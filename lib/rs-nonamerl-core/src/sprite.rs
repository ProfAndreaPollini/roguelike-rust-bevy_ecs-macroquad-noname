#![allow(dead_code)]
use std::collections::HashMap;

use bevy_ecs::system::Resource;
use serde::Deserialize;

use macroquad::{
    prelude::{IVec4, Rect},
    texture::{load_texture, FilterMode, Texture2D},
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AddSpriteOptions {
    pub gap: (u32, u32),
    pub size: (u32, u32),
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub pos: Rect,
    pub spritesheet: u8,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct SpriteContainer {
    pub textures: Vec<Texture2D>,
    pub spritesheets: HashMap<String, Sprite>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SpriteRect(u32, u32, u32, u32);

impl From<SpriteRect> for Rect {
    fn from(sprite_rect: SpriteRect) -> Self {
        Rect::new(
            sprite_rect.0 as f32,
            sprite_rect.1 as f32,
            sprite_rect.2 as f32,
            sprite_rect.3 as f32,
        )
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SpriteConfig {
    pub name: String,
    pub pos: (u32, u32),

    #[serde(default)]
    pub options: AddSpriteOptions,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SpriteSheetConfig {
    pub sprites: HashMap<String, Vec<SpriteConfig>>,
    pub defaults: HashMap<String, AddSpriteOptions>,
}

impl SpriteContainer {
    /// Creates a new sprite container from a spritesheet.
    pub async fn from_spritesheet(spritesheet_path: &str) -> Self {
        let texture = load_texture(spritesheet_path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        Self {
            textures: vec![texture],
            ..Default::default()
        }
    }

    pub async fn add_spritesheet(&mut self, spritesheet_path: &str) -> usize {
        let texture = load_texture(spritesheet_path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub async fn from_config(config_path: &str) -> Self {
        let config_content =
            &std::fs::read_to_string(config_path).expect("Failed to read config file");

        let config: SpriteSheetConfig = serde_json::from_str(config_content)
            .unwrap_or_else(|_| panic!("Failed to parse {}", config_path));

        let mut sprite_container = Self::default();

        for (name, sprite_config) in config.sprites {
            sprite_container.add_spritesheet(&name).await;
            let spritesheet_defaults = config.defaults.get(&name).unwrap();
            for sprite in sprite_config {
                let sprite_options = if sprite.options.size == (0, 0) {
                    Some(spritesheet_defaults.clone())
                } else {
                    Some(sprite.options)
                };
                println!("adding sprite: {:?}", sprite.name);
                sprite_container.add_sprite(&sprite.name, sprite.pos, sprite_options);
            }
        }

        sprite_container
    }

    pub fn add_texture(&mut self, texture: Texture2D) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn add_sprite(
        &mut self,
        name: &str,
        sprite_cell: (u32, u32),
        add_sprite_options: Option<AddSpriteOptions>,
    ) {
        let mut pos = Rect::default();
        if let Some(add_sprite_optione) = add_sprite_options {
            pos.x = sprite_cell.0 as f32
                * (add_sprite_optione.size.0 + add_sprite_optione.gap.0) as f32;
            pos.y = sprite_cell.1 as f32
                * (add_sprite_optione.size.1 + add_sprite_optione.gap.1) as f32;
            pos.w = add_sprite_optione.size.0 as f32;
            pos.h = add_sprite_optione.size.1 as f32;
        }

        self.spritesheets.insert(
            name.to_owned(),
            Sprite {
                pos,
                spritesheet: 0,
            },
        );
    }

    pub fn get_sprite(&self, name: &str) -> (&Rect, &Texture2D) {
        // println!("getting sprite: {:?}", name);
        let sprite = self.spritesheets.get(name).unwrap();
        (&sprite.pos, &self.textures[sprite.spritesheet as usize])
    }
}
