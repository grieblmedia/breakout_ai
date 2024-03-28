use bevy::math::Vec2;

use crate::game_action::GameAction;

/// Struktur, die einen Spielzustand darstellt.
#[derive(Clone, Resource)]
pub struct GameState {
    pub ball_position: Vec2,
    pub paddle_position: Vec2,
    pub velocity: Vec2,
    pub action: GameAction, // Aktion, die in diesem Zustand unternommen wurde
    pub reward: f32,        // Belohnung, die nach Ausführung der Aktion erhalten wurde
}

impl GameState {
    // Initialisiert die RL-Umgebung mit dem Startzustand des Spiels
    pub fn new() -> Self {
        GameState {
            ball_position: Vec2::new(0.0, -50.0),
            paddle_position: Vec2::new(0.0, super::GAP_BETWEEN_PADDLE_AND_FLOOR),
            velocity: Vec2::new(super::INITIAL_BALL_DIRECTION.x * super::BALL_SPEED, 0.0),
            action: GameAction::Stay,
            reward: 0.0,
        }
    }

    // Setters
    pub fn set_ball_position(&mut self, ball_position: Vec2) {
        self.ball_position = ball_position;
    }

    pub fn set_paddle_position(&mut self, paddle_position: Vec2) {
        self.paddle_position = paddle_position;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn set_action(&mut self, action: GameAction) {
        self.action = action;
    }

    pub fn set_reward(&mut self, reward: f32) {
        self.reward = reward;
    }

    // Getter für den gesamten GameState
    pub fn get_state(&self) -> Self {
        self.clone()
    }
}
