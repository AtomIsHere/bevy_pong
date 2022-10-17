mod player;
mod ball;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::player::{Player1, player1_movement_system, Player2, player2_movement_system};
use crate::ball::{Ball, BallDirection, move_ball_system};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player1_movement_system)
                .with_system(player2_movement_system)
                .with_system(move_ball_system)
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let bar_sprite = Sprite {
        color: Color::rgb(1., 1., 1.),
        custom_size: Some(Vec2::new(8., 50.)),
        ..default()
    };

    commands.spawn_bundle(SpriteBundle {
        sprite: bar_sprite.clone(),
        ..default()
    }).insert(Player1 {
        movement_speed: 10.
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: bar_sprite.clone(),
        ..default()
    }).insert(Player2 {
        movement_speed: 10.
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(8., 8.)),
            ..default()
        },
        ..default()
    }).insert(Ball {
        movement_speed: 8.
    }).insert(BallDirection {
        facing: 1.,
    });
}