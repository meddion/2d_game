use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::{
    states::MainState,
    systems::{
        ball::{BallMoveSys, BounceSys},
        players::PlayersMoveSys,
        score::ScoreSys,
    },
};

mod components;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("configs");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(config_path.join("key_bindings.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_path.join("display.ron"))?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        // Systems
        .with(BallMoveSys, "ball_sys", &[])
        .with(PlayersMoveSys, "players_sys", &["input_system"])
        .with(
            BounceSys,
            "bounce_sys",
            &["players_sys", "ball_sys"], // dependencies
        )
        .with(ScoreSys, "score_sys", &["ball_sys"]);

    let mut game = Application::new(app_root.join("assets"), MainState, game_data)?;
    game.run();

    Ok(())
}
