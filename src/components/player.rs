use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    window::ScreenDimensions,
};

pub const PLAYER_W: f32 = 48.0;
pub const PLAYER_H: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 230.0;

pub struct Player {
    pub width: f32,
    pub height: f32,
    pub side: Side,
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Player {
    pub fn new(side: Side) -> Player {
        Player {
            width: PLAYER_W,
            height: PLAYER_H,
            side,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_players(world: &mut World, sprite_sheet: Handle<SpriteSheet>, sprite_number: usize) {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number,
    };
    let w = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.width()
    };
    let y = PLAYER_H * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(PLAYER_W * 0.5, y, 0.0);
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Left))
        .with(transform)
        .build();
    let mut transform = Transform::default();
    transform.set_translation_xyz(w - PLAYER_W * 0.5, y, 0.0);
    world
        .create_entity()
        .with(sprite_render)
        .with(Player::new(Side::Right))
        .with(transform)
        .build();
}
