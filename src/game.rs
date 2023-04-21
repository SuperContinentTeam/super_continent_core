use crate::state::state::GameState;

pub async fn game_loop(game_state: &GameState) {
    loop {
        {
            game_state.time_flow();
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}