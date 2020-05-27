use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
    window::ScreenDimensions,
};

use crate::components::{
    ball::Ball,
    score::{ScoreBoard, ScoreText},
};

#[derive(SystemDesc)]
pub struct ScoreSys;

impl<'a> System<'a> for ScoreSys {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, UiText>,
        Write<'a, ScoreBoard>,
        ReadExpect<'a, ScoreText>,
        ReadExpect<'a, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (mut balls, mut transforms, mut ui_text, mut scores, score_text, dim): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let (w, h) = (dim.width() * 0.5, dim.height() * 0.5);

            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball_y <= ball.radius {
                if ball_x <= w {
                    scores.score_right = (scores.score_right + 1).min(5000);
                    if let Some(text) = ui_text.get_mut(score_text.score2) {
                        text.text = scores.score_right.to_string();
                    }
                } else {
                    scores.score_left = (scores.score_left + 1).min(5000);
                    if let Some(text) = ui_text.get_mut(score_text.score1) {
                        text.text = scores.score_left.to_string();
                    }
                }
                // restart at the center
                transform.set_translation_x(w);
                transform.set_translation_y(h);
                ball.velocity[0] = -ball.velocity[0];
                ball.velocity[1] = 0.0;
            }
        }
    }
}
