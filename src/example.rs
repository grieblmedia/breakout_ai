use bevy::prelude::*;
use std::collections::HashMap;

// Diese Struktur repräsentiert die Umgebung, in der der RL-Agent interagiert.
struct RlEnvironment {
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
    fn new() -> Self {
        RlEnvironment {
            state: GameState {
                ball_position: Vec2::new(0.0, -50.0),
                paddle_position: Vec2::new(0.0, GAP_BETWEEN_PADDLE_AND_FLOOR),
                velocity: INITIAL_BALL_DIRECTION * BALL_SPEED,
                bricks_remaining: 0, // Dies sollte auf die anfängliche Anzahl der Ziegel gesetzt werden
            },
            rewards: HashMap::from([
                ("brick_hit".to_string(), 1.0),
                ("ball_lost".to_string(), -1.0),
                ("game_won".to_string(), 10.0),
                ("game_lost".to_string(), -10.0),
            ]),
        }
    }

    // Aktualisiert den Zustand der Umgebung basierend auf dem Spielgeschehen
    fn update_state(&mut self, event: &GameEvent) {
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
                self.state.bricks_remaining -= 1;
            }
            _ => {}
        }
    }

    // Berechnet die Belohnung für das gegebene Ereignis
    fn calculate_reward(&self, event: &GameEvent) -> f32 {
        match event {
            GameEvent::BrickDestroyed => *self.rewards.get("brick_hit").unwrap(),
            GameEvent::BallLost => *self.rewards.get("ball_lost").unwrap(),
            GameEvent::GameWon => *self.rewards.get("game_won").unwrap(),
            GameEvent::GameLost => *self.rewards.get("game_lost").unwrap(),
            _ => 0.0,
        }
    }
}

// Definiert verschiedene Arten von Ereignissen, die während des Spiels auftreten können
enum GameEvent {
    BallPosition(f32, f32),
    PaddlePosition(f32),
    Velocity(f32, f32),
    BrickDestroyed,
    BallLost,
    GameWon,
    GameLost,
}

// Beispiel für die Verwendung der RlEnvironment in einer Systemfunktion
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