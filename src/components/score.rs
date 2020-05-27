use amethyst::{
    assets::Loader,
    ecs::prelude::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub score1: Entity,
    pub score2: Entity,
}

impl ScoreText {
    fn new(score1: Entity, score2: Entity) -> ScoreText {
        ScoreText { score1, score2 }
    }
}

pub fn init_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/font1.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let score1 = world
        .create_entity()
        .with(UiTransform::new(
            "P1".to_string(),
            Anchor::TopMiddle,
            Anchor::Middle,
            -50.0,
            -50.0,
            1.0,
            200.0,
            50.0,
        ))
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [0.5, 0.7, 1.0, 1.0],
            50.0,
        ))
        .build();

    let score2 = world
        .create_entity()
        .with(UiTransform::new(
            "P2".to_string(),
            Anchor::TopMiddle,
            Anchor::Middle,
            50.0,
            -50.0,
            1.0,
            200.0,
            50.0,
        ))
        .with(UiText::new(font, "0".to_string(), [1., 1., 1., 1.], 50.))
        .build();

    world.insert(ScoreText::new(score1, score2));
}
