use bevy::prelude::{Component, Res, Text, Transform, Windows};
use crate::{Ball, BallDirection, Query, Without};

#[derive(Component)]
pub struct PlayerScore {
    pub score: u32
}

#[derive(Component)]
pub struct Player1ScoreDisplay;
#[derive(Component)]
pub struct Player2ScoreDisplay;

pub fn player_score_system(windows: Res<Windows>, mut player1_query: Query<(&Player1ScoreDisplay, &mut PlayerScore, &mut Text), Without<Player2ScoreDisplay>>, mut player2_query: Query<(&Player2ScoreDisplay, &mut PlayerScore, &mut Text), Without<Player1ScoreDisplay>>, mut ball_query: Query<(&Ball, &mut Transform, &mut BallDirection)>) {
    let (_, mut player1_score, mut player1_text) = player1_query.single_mut();
    let (_, mut player2_score, mut player2_text) = player2_query.single_mut();
    let (_, mut ball, mut ball_direction) = ball_query.single_mut();

    let window = windows.get_primary().unwrap();

    let max_x = window.width()/2.;
    let min_x = -max_x;

    if ball.translation.x >= max_x {
        player2_score.score += 1;
        player2_text.sections[0].value = player2_score.score.to_string();

        reset_ball_pos(&mut ball, &mut ball_direction);
    } else if ball.translation.x <= min_x {
        player1_score.score += 1;
        player1_text.sections[0].value = player1_score.score.to_string();

        reset_ball_pos(&mut ball, &mut ball_direction);
    }
}

pub fn score_movement_system(windows: Res<Windows>, mut player1_query: Query<(&Player1ScoreDisplay, &mut Transform), Without<Player2ScoreDisplay>>, mut player2_query: Query<(&Player2ScoreDisplay, &mut Transform), Without<Player1ScoreDisplay>>) {
    let (_, mut player1_score) = player1_query.single_mut();
    let (_, mut player2_score) = player2_query.single_mut();

    let window = windows.get_primary().unwrap();

    let y_pos = window.height()/4.;
    let rel_x = window.width()/4.;

    player1_score.translation.x = rel_x;
    player1_score.translation.y = y_pos;

    player2_score.translation.x = -rel_x;
    player2_score.translation.y = y_pos;
}

fn reset_ball_pos(ball: &mut Transform, ball_dir: &mut BallDirection) {
    ball.translation.x = 0.;
    ball.translation.y = 0.;
    ball_dir.facing = 0.;
}