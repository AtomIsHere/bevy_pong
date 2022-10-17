use std::f32::consts::PI;
use bevy::prelude::*;

const REVOLUTION_RAD: f32 = 2. * PI;

#[derive(Component)]
pub struct Ball {
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct BallDirection {
    pub facing: f32,
}

pub fn move_ball_system(windows: Res<Windows>, mut query: Query<(&Ball, &mut BallDirection, &mut Transform)>) {
    let (ball, mut direction, mut transform) = query.single_mut();
    let window = windows.get_primary().unwrap();

    let max_height: f32 = window.height()/2.;
    let min_height: f32 = -max_height;

    if max_height <= transform.translation.y || min_height >= transform.translation.y {
        direction.facing = REVOLUTION_RAD - direction.facing;
    }

    transform.translation.x += ball.movement_speed * direction.facing.cos();
    transform.translation.y += ball.movement_speed * direction.facing.sin();
}

