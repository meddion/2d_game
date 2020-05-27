use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    window::ScreenDimensions,
};

use rand::{self, Rng};

use crate::components::{
    ball::Ball,
    player::{Player, Side},
};

#[derive(SystemDesc)]
pub struct BallMoveSys;

pub const GRAVITY: f32 = -80.0;

impl<'a> System<'a> for BallMoveSys {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut balls, mut transforms, time): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            transform.append_translation_xyz(
                ball.velocity[0] * time.delta_seconds(),
                (ball.velocity[1] + time.delta_seconds() * GRAVITY * 0.5) * time.delta_seconds(),
                0.0,
            );
            ball.velocity[1] += time.delta_seconds() * GRAVITY;
        }
    }
}

#[derive(SystemDesc)]
pub struct BounceSys;
impl<'a> System<'a> for BounceSys {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadExpect<'a, ScreenDimensions>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
    );
    fn run(&mut self, (mut balls, dim, players, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let (ball_x, ball_y) = (transform.translation().x, transform.translation().y);
            // bounce from borders
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= (dim.height() - ball.radius) && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
            } else if (ball_x <= (ball.radius) && ball.velocity[0] < 0.0)
                || (ball_x >= (dim.width() - ball.radius) && ball.velocity[0] > 0.0)
            {
                ball.velocity[0] = -ball.velocity[0];
            }

            for (player, player_transform) in (&players, &transforms).join() {
                let player_x = player_transform.translation().x - (player.width * 0.5);
                let player_y = player_transform.translation().y - (player.height * 0.5);
                if is_inside_zone(
                    ball_x,
                    ball_y,
                    player_x - ball.radius,
                    player_y - ball.radius,
                    player_x + player.width + ball.radius,
                    player_y + player.height + ball.radius,
                ) && ball.velocity[1] < 0.0
                {
                    ball.velocity[1] = -ball.velocity[1];
                    let mut rng = rand::thread_rng();
                    match player.side {
                        Side::Left => {
                            ball.velocity[0] = ball.velocity[0].abs() * rng.gen_range(0.5, 1.5)
                        }
                        Side::Right => {
                            ball.velocity[0] = -ball.velocity[0].abs() * rng.gen_range(0.5, 1.5)
                        }
                    }
                }
            }
        }
    }
}

#[inline]
fn is_inside_zone(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
