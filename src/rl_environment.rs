use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    BOTTOM_WALL, BRICK_SIZE, GAP_BETWEEN_BRICKS, GAP_BETWEEN_BRICKS_AND_CEILING,
    GAP_BETWEEN_BRICKS_AND_SIDES, GAP_BETWEEN_PADDLE_AND_BRICKS, GAP_BETWEEN_PADDLE_AND_FLOOR,
    LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

// Diese Struktur repräsentiert die Umgebung, in der der RL-Agent interagiert.
#[derive(Resource)]
pub struct RlEnvironment {
    // Speichert den aktuellen Zustand der Umgebung
    state: GameState,

    // Speichert die Belohnungen für bestimmte Aktionen
    rewards: HashMap<String, f32>,
}

// Repräsentiert den Spielzustand, den der RL-Agent beobachtet
#[derive(Clone)]
struct GameState {
    ball_position: Vec2,
    paddle_position: Vec2,
    velocity: Vec2,
    bricks_remaining: usize,
}

impl RlEnvironment {
    // Initialisiert die RL-Umgebung mit dem Startzustand des Spiels
    pub fn new() -> Self {
        let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
        let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
        let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
        let total_height_of_bricks =
            TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;
        let n_columns =
            (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
        let n_rows =
            (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;

        RlEnvironment {
            state: GameState {
                ball_position: Vec2::new(0.0, -50.0),
                paddle_position: Vec2::new(0.0, super::GAP_BETWEEN_PADDLE_AND_FLOOR),
                velocity: super::INITIAL_BALL_DIRECTION * super::BALL_SPEED,
                bricks_remaining: n_columns * n_rows, // Dies sollte auf die anfängliche Anzahl der Ziegel gesetzt werden
            },
            rewards: HashMap::from([
                ("brick_hit".to_string(), 1.0),
                ("bottom_wall".to_string(), -1.0),
                ("game_won".to_string(), 10.0),
                ("game_lost".to_string(), -10.0),
            ]),
        }
    }

    // Aktualisiert den Zustand der Umgebung basierend auf dem Spielgeschehen
    pub fn update_state(&mut self, event: &GameEvent) {
        match event {
            GameEvent::BallPosition(x, y) => {
                self.state.ball_position = Vec2::new(*x, *y);
            }
            GameEvent::PaddlePosition(x) => {
                self.state.paddle_position.x = *x;
            }
            GameEvent::Velocity(x, y) => {
                self.state.velocity = Vec2::new(*x, *y);
            }
            GameEvent::BrickDestroyed => {
                if self.state.bricks_remaining > 0 {
                    self.state.bricks_remaining -= 1;
                } else {
                    self.state.bricks_remaining = 0;
                }
            }
            _ => {}
        }
    }

    // Berechnet die Belohnung für das gegebene Ereignis
    pub fn calculate_reward(&self, event: &GameEvent) -> f32 {
        match event {
            GameEvent::BrickDestroyed => *self.rewards.get("brick_hit").unwrap(),
            GameEvent::Bottomwall => *self.rewards.get("bottom_wall").unwrap(),
            GameEvent::GameWon => *self.rewards.get("game_won").unwrap(),
            GameEvent::GameLost => *self.rewards.get("game_lost").unwrap(),
            _ => 0.0,
        }
    }
}

// Definiert verschiedene Arten von Ereignissen, die während des Spiels auftreten können
pub enum GameEvent {
    BallPosition(f32, f32),
    PaddlePosition(f32),
    Velocity(f32, f32),
    BrickDestroyed,
    Bottomwall,
    GameWon,
    GameLost,
}

/*Beispiel für die Verwendung der RlEnvironment in einer Systemfunktion
fn game_event_listener(
    mut rl_env: ResMut<RlEnvironment>,
    // Fügen Sie Parameter hier hinzu, um Spielereignisse zu empfangen
) {
    // Beispiel: Ein Ziegel wurde getroffen
    let event = GameEvent::BrickDestroyed;
    rl_env.update_state(&event);
    let reward = rl_env.calculate_reward(&event);
    println!("Reward for event: {}", reward);

    // Hier würden Sie die Belohnung und den aktualisierten Zustand an Ihr RL-Modell senden
}
*/
