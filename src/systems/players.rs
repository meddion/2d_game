use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::player::{Player, Side, PLAYER_SPEED, PLAYER_W};

#[derive(SystemDesc)]
pub struct PlayersMoveSys;

impl<'a> System<'a> for PlayersMoveSys {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, ScreenDimensions>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, players, dim, input, time): Self::SystemData) {
        let half_w = dim.width() * 0.5;
        for (player, transform) in (&players, &mut transforms).join() {
            let movement = match player.side {
                Side::Left => input.axis_value("left_player"),
                Side::Right => input.axis_value("right_player"),
            };

            if let Some(move_direction) = movement {
                let player_x = transform.translation().x;
                let left_limit = match player.side {
                    Side::Left => 0.0,
                    Side::Right => half_w,
                };
                let scaled_amount = time.delta_seconds() * PLAYER_SPEED * move_direction as f32;
                let player_pos = (player_x + scaled_amount)
                    .max(left_limit + PLAYER_W)
                    .min(left_limit + half_w - PLAYER_W);
                transform.set_translation_x(player_pos);
            }
        }
    }
}
