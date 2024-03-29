use std::collections::HashMap;

pub enum GameEvent {
    BallPosition(f32, f32),
    PaddlePosition(f32),
    Velocity(f32, f32),
    BrickDestroyed,
    Bottomwall,
    GameWon,
    GameLost,
}