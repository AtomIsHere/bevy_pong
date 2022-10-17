use bevy::prelude::{Component, Input, Res, Window, Windows};
use crate::{KeyCode, Query, Transform};

#[derive(Component)]
pub struct Player1 {
    pub movement_speed: f32
}

#[derive(Component)]
pub struct Player2 {
    pub movement_speed: f32
}

pub fn player1_movement_system(keyboard_input: Res<Input<KeyCode>>, windows: Res<Windows>, mut query: Query<(&Player1, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    let window = windows.get_primary().unwrap();

    let max_bound = window.width()/2.;
    transform.translation.x = max_bound - 100.;

    move_player(keyboard_input, window, player.movement_speed, &mut transform, KeyCode::Up, KeyCode::Down);
}

pub fn player2_movement_system(keyboard_input: Res<Input<KeyCode>>, windows: Res<Windows>, mut query: Query<(&Player2, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    let window = windows.get_primary().unwrap();

    let min_bound = window.width()/-2.;
    transform.translation.x = min_bound + 100.;

    move_player(keyboard_input, window, player.movement_speed, &mut transform, KeyCode::W, KeyCode::S);
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, window: &Window, movement_speed: f32, transform: &mut Transform, up: KeyCode, down: KeyCode) {
    if keyboard_input.pressed(up) {
        let move_to = transform.translation.y + movement_speed;
        transform.translation.y = min(move_to, window.height()/2.);
    }

    if keyboard_input.pressed(down) {
        let move_to = transform.translation.y - movement_speed;
        transform.translation.y = max(move_to, window.height()/-2.);
    }
}

fn max(v1: f32, v2: f32) -> f32 {
    return if v1 >= v2 {
        v1
    } else {
        v2
    }
}

fn min(v1: f32, v2: f32) -> f32 {
    return if v1 <= v2 {
        v1
    } else {
        v2
    }
}