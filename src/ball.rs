use std::f32::consts::PI;
use std::ops::Range;
use bevy::prelude::*;
use crate::{Player1, Player2, PLAYER_DIMENSIONS};
use crate::collision::Collidable;

const REVOLUTION_RAD: f32 = 2. * PI;
pub const BALL_SIZE: f32 = 8.;

const BALL_VEC: Vec2 = Vec2 {x: BALL_SIZE, y: BALL_SIZE};

const PI_OVER_180: f32 = PI/180.;

const RIGHT_PADDLE_ANGLE_RANGE: Range<f32> = 110.*PI_OVER_180..250.*PI_OVER_180;
const LEFT_PADDLE_ANGLE_RANGE: Range<f32> = 70.*PI_OVER_180..-70.*PI_OVER_180;

const PADDLE_CENTER: f32 = PLAYER_DIMENSIONS.y/2.;

#[derive(Component)]
pub struct Ball {
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct BallDirection {
    pub facing: f32,
}

pub fn move_ball_system(windows: Res<Windows>, player1_query: Query<(&Player1, &Transform), Without<Ball>>, player2_query: Query<(&Player2, &Transform), Without<Ball>>, mut ball_query: Query<(&Ball, &mut BallDirection, &mut Transform)>) {
    let (ball, mut direction, mut ball_transform) = ball_query.single_mut();

    let (_, player1) = player1_query.single();
    let (_, player2) = player2_query.single();

    let window = windows.get_primary().unwrap();

    let max_height: f32 = window.height()/2.;
    let min_height: f32 = -max_height;

    if max_height <= ball_transform.translation.y || min_height >= ball_transform.translation.y {
        direction.facing = REVOLUTION_RAD - direction.facing;
    }

    if ball_transform.translation.x > 0. {
        if ball_transform.collides(BALL_VEC, &player1, PLAYER_DIMENSIONS) {
            let paddle_range = paddle_range(player1.translation.y);
            direction.facing = scale_range(ball_transform.translation.y, paddle_range, RIGHT_PADDLE_ANGLE_RANGE);
        }
    } else {
        if ball_transform.collides(BALL_VEC, &player2, PLAYER_DIMENSIONS) {
            let paddle_range = paddle_range(player2.translation.y);
            direction.facing = scale_range(ball_transform.translation.y, paddle_range, LEFT_PADDLE_ANGLE_RANGE);
        }
    }

    ball_transform.translation.x += ball.movement_speed * direction.facing.cos();
    ball_transform.translation.y += ball.movement_speed * direction.facing.sin();
}

fn scale_range(num: f32, range: Range<f32>, scalar: Range<f32>) -> f32 {
    return (((num - range.start) * (scalar.end - scalar.start)) / (range.end - range.start)) + scalar.start;
}

fn paddle_range(y: f32) -> Range<f32> {
    y + PADDLE_CENTER..y - PADDLE_CENTER
}