mod player;
mod ball;
mod collision;
mod score;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::player::{Player1, player1_movement_system, Player2, player2_movement_system, PLAYER_DIMENSIONS};
use crate::ball::{Ball, BALL_SIZE, BallDirection, move_ball_system};
use crate::score::{Player1ScoreDisplay, Player2ScoreDisplay, player_score_system, PlayerScore, score_movement_system};

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
                .with_system(player_score_system)
                .with_system(score_movement_system)
        )
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    commands.spawn_bundle(Camera2dBundle::default());

    let bar_sprite = Sprite {
        color: Color::rgb(1., 1., 1.),
        custom_size: Some(PLAYER_DIMENSIONS.clone()),
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
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..default()
        },
        ..default()
    }).insert(Ball {
        movement_speed: 8.
    }).insert(BallDirection {
        facing: 180.*(PI/180.),
    });

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("0", text_style.clone()).with_alignment(text_alignment),
        ..default()
    }).insert(Player1ScoreDisplay).insert(PlayerScore {
        score: 0
    });

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("0", text_style.clone()).with_alignment(text_alignment),
        ..default()
    }).insert(Player2ScoreDisplay).insert(PlayerScore {
        score: 0
    });
}