use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, World},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    window::ScreenDimensions,
};

pub const BALL_VEL: [f32; 2] = [200.0, 0.0];
pub const BALL_RAD: f32 = 16.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            radius: BALL_RAD,
            velocity: BALL_VEL,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>, sprite_number: usize) {
    let (w, h) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(w * 0.5, h * 0.5, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball::new())
        .with(transform)
        .build();
}
