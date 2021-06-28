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
        utils.fill(0, 0, utils.width, utils.height, ' ', Color::Black);
    }

    fn on_user_update(&mut self, utils: &mut Utils, elapsed_time: f64) {
        for y in 0..(utils.height / 2) {
            for x in 0..utils.width {
                let middle = 0.5;
                let mut road_width = 0.6;
                let clip_width = road_width * 0.15;
                road_width /= 2.0;

                let left_grass = (middle - road_width - clip_width) * utils.width as f64;
                let right_grass = (middle + road_width + clip_width) * utils.width as f64;
                let left_clip = (middle - road_width) * utils.width as f64;
                let right_clip = (middle + road_width) * utils.width as f64;

                let y_flip = (utils.height / 2) + y;
                match x {
                    x if x < left_grass as usize => utils.draw(x, y_flip, '█', Color::Green),
                    x if x >= left_grass as usize && x < left_clip as usize => utils.draw(x, y_flip, '█', Color::Red),
                    x if x >= left_clip as usize && x < right_clip as usize => utils.draw(x, y_flip, '█', Color::DarkGrey),
                    x if x >= right_clip as usize && x < right_grass as usize => utils.draw(x, y_flip, '█', Color::Red),
                    x if x >= right_grass as usize => utils.draw(x, y_flip, '█', Color::Green),
                    _ => ()
                } 
            }
        }
    }
}

fn main() -> Result<()> {
    let rules = RacerRules::new();
    let mut game = ConsoleGameEngine::new(100, 160, rules);
    game.construct_console()?;
    game.start()?;

    Ok(())
}