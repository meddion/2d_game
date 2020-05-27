use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use crate::components::{ball::init_ball, player::init_players, score::init_scoreboard};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        init_camera(world);
        let sprite_sheet =
            load_sprite_sheet(world, "textures/spritesheet.png", "textures/sprites.ron");
        init_players(world, sprite_sheet.clone(), 0);
        init_ball(world, sprite_sheet, 1);
        init_scoreboard(world);
    }
}

// init_camera adds a camera with orthographic projection to the game world.
fn init_camera(world: &mut World) {
    let (w, h) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut pos = Transform::default();
    pos.set_translation_xyz(w * 0.5, h * 0.5, 1.0);
    world
        .create_entity()
        .with(pos)
        .with(Camera::standard_2d(w, h))
        .build();
}

// Loads the resources asynchronously.
fn load_sprite_sheet(world: &mut World, texture: &str, meta: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            texture,
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };
    let loader = world.read_resource::<Loader>();
    loader.load(
        meta,
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}
