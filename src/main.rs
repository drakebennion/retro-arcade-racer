use olc_rust_game_engine::{Color, ConsoleGameEngine, Utils, Result, Rules};

struct RacerRules {
    car_distance: f64,
    car_position: f64
}

impl RacerRules {
    fn new() -> Self {
        RacerRules{
            car_distance : 0.0,
            car_position : 0.0
        }
    }
}

impl Rules for RacerRules {
    fn on_user_create(&mut self, utils: &mut Utils) {
        utils.fill(0, 0, utils.width, utils.height, ' ', Color::Black);
    }

    fn on_user_update(&mut self, utils: &mut Utils, elapsed_time: f64) {
        if utils.keys.contains(&38) {
            self.car_distance += 500.0 * elapsed_time;
        }

        // draw road, clip, and grass
        for y in 0..(utils.height / 2) {
            for x in 0..utils.width {
                let perspective = y as f64 / (utils.height as f64 / 2.0);
                let middle = 0.5;
                let mut road_width = 0.1 + perspective * 0.8;
                let clip_width = road_width * 0.15;
                road_width /= 2.0;

                let left_grass = (middle - road_width - clip_width) * utils.width as f64;
                let right_grass = (middle + road_width + clip_width) * utils.width as f64;
                let left_clip = (middle - road_width) * utils.width as f64;
                let right_clip = (middle + road_width) * utils.width as f64;

                let y_flip = (utils.height / 2) + y;

                let grass_arg: f64 = 20.0 * (1.0 - perspective).powf(3.0) + self.car_distance * 0.1;
                let grass_color = if grass_arg.sin() > 0.0 { Color::Green } else { Color::DarkGreen };
                match x {
                    x if x < left_grass as usize => utils.draw(x, y_flip, '█', grass_color),
                    x if x >= left_grass as usize && x < left_clip as usize => utils.draw(x, y_flip, '█', Color::Red),
                    x if x >= left_clip as usize && x < right_clip as usize => utils.draw(x, y_flip, '█', Color::DarkGrey),
                    x if x >= right_clip as usize && x < right_grass as usize => utils.draw(x, y_flip, '█', Color::Red),
                    x if x >= right_grass as usize => utils.draw(x, y_flip, '█', grass_color),
                    _ => ()
                } 
            }
        }

        // draw car
        let car_pos = utils.width / 2 + ( utils.width * self.car_position as usize / 2) - 4;
        // todo: make this car more car-y
        utils.fill(car_pos, 80, car_pos + 4, 84, '#', Color::Black);
    }
}

fn main() -> Result<()> {
    let rules = RacerRules::new();
    let mut game = ConsoleGameEngine::new(100, 160, rules);
    game.construct_console()?;
    game.start(true)?;

    Ok(())
}