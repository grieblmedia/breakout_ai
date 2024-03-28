use std::fs::OpenOptions;
use std::io::Write;


/// Fügt einen Spielzustand zur Datensammlung hinzu.
pub fn log_state(game_state: &GameState) {
    let file_path = "game_data.csv";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    // Konvertiert GameAction zu einem einfacheren Format für die Aufzeichnung
    let action = match game_state.action {
        GameAction::MoveLeft => "Left",
        GameAction::MoveRight => "Right",
        GameAction::Stay => "Stay",
    };

    // Erstellt eine Zeichenkette, die den Spielzustand repräsentiert
    let line = format!(
        "{},{},{},{},{},{}\n",
        game_state.ball_position.x, game_state.ball_position.y,
        game_state.paddle_position.x, game_state.velocity.x, game_state.velocity.y,
        action
    );

    // Schreibt die Zeichenkette in die Datei
    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Konnte Spielzustand nicht aufzeichnen: {}", e);
    }
}

/* Beispiel für die Verwendung der `log_state` Funktion.
fn example_usage() {
    let game_state = GameState {
        ball_position: Vec2::new(0.5, -0.5),
        paddle_position: Vec2::new(0.0, -1.0),
        velocity: Vec2::new(0.1, -0.1),
        action: GameAction::MoveRight,
        reward: 1.0, // Beispielwert
    };

    log_state(&game_state);
}
*/