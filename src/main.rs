use olc_rust_game_engine::{Color, ConsoleGameEngine, Utils, Result, Rules};

struct RacerRules {

}

impl RacerRules {
    fn new() -> Self {
        RacerRules{}
    }
}

impl Rules for RacerRules {
    fn on_user_create(&mut self, utils: &mut Utils) {

    }

    fn on_user_update(&mut self, utils: &mut Utils, elapsed_time: f64) {

    }
}

fn main() -> Result<()> {
    let rules = RacerRules::new();
    let mut game = ConsoleGameEngine::new(100, 160, rules);
    game.construct_console()?;
    game.start()?;

    Ok(())
}