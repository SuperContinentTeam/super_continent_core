use crate::state::state::GameState;

pub async fn game_loop(game_state: &GameState) {
    loop {
        if !*game_state.pause.read().unwrap() {
            game_state.time_flow();
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

pub async fn command_executor(game_state: &GameState, cmd: &str) -> String {
    match cmd {
        "date" => {
            let tick = game_state.tick.read().unwrap();
            let (year, month, day) = tick.datetime();

            format!("{}年{}月{}日", year, month, day)
        }
        "pause" => {
            let read_pause = { *game_state.pause.read().unwrap() };

            let mut write_pause = game_state.pause.write().unwrap();
            *write_pause = !read_pause;

            "success".to_string()
        }
        _ => format!("Command:{} not found", cmd),
    }
}
