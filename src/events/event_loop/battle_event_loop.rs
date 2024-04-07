use super::EventLoop;

#[derive(Clone)]
pub struct BattleEventLoop {
    pub interval: u64,
}

impl EventLoop for BattleEventLoop {
    fn is_event_loop_active(&self) -> bool {
        true
    }

    fn progress_event_loop(&mut self) -> String {
        "Hello world?".to_string()
    }

    // fn progress_event_loop(&mut self, player: &mut Player) -> String {
    //     let mut result = String::new();
    //     let enemy = &mut self.enemy;

    //     match self.attack_turn {
    //         Turn::Player => {
    //             result = player.attack(enemy);

    //             if !enemy.is_alive() {
    //                 result = format!("You defeated {}!", enemy.name);
    //                 // if let Some(reward_text) = self.game_state.go_to_next_encounter(player) {
    //                 //     result = format!("{}\n{}", result, reward_text);
    //                 // }

    //                 self.attack_turn = Turn::Player;
    //             } else {
    //                 self.attack_turn = Turn::Enemy;
    //             }
    //             result
    //         }
    //         Turn::Enemy => {
    //             result = enemy.attack(player);

    //             if player.is_alive() {
    //                 // self.game_state.state = PlayerState::GameOver;
    //                 result = format!("{} died!\nGame Over...", player.name);
    //             } else {
    //                 self.attack_turn = Turn::Player;
    //             }
    //             result
    //         }
    //     }
    // }
}
