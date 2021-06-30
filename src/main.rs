use olc_rust_game_engine::{Color, ConsoleGameEngine, Result, Rules, Utils};

struct RacerRules {
    car_distance: f64,
    car_position: f64,
    curvature: f64,
    player_curvature: f64,
    speed: f64,
    track: Vec<(f64, f64)>,
    track_curvature: f64,
}

impl RacerRules {
    fn new() -> Self {
        RacerRules {
            car_distance: 0.0,
            car_position: 0.0,
            curvature: 0.0,
            player_curvature: 0.0,
            speed: 0.0,
            track: vec![],
            track_curvature: 0.0,
        }
    }
}

impl Rules for RacerRules {
    fn on_user_create(&mut self, utils: &mut Utils) {
        self.track.push((0.0, 10.0));
        self.track.push((0.0, 250.0));
        self.track.push((0.5, 200.0));
        self.track.push((1.0, 200.0));
        self.track.push((0.5, 175.0));
        self.track.push((0.0, 200.0));
        self.track.push((-0.5, 100.0));
        self.track.push((-1.0, 100.0));
        self.track.push((0.0, 200.0));
        self.track.push((-1.0, 200.0));
        self.track.push((1.0, 200.0));
        self.track.push((0.0, 200.0));
        self.track.push((0.2, 500.0));
        self.track.push((0.0, 200.0));
        utils.fill(0, 0, utils.width, utils.height, ' ', Color::Black);
    }

    fn on_user_update(&mut self, utils: &mut Utils, elapsed_time: f64) {
        if utils.keys.contains(&38) {
            self.speed += 2.0 * elapsed_time;
        } else {
            self.speed -= 1.0 * elapsed_time;
        }

        if utils.keys.contains(&37) {
            self.player_curvature -= 0.7 * elapsed_time;
        }

        if utils.keys.contains(&39) {
            self.player_curvature += 0.7 * elapsed_time;
        }

        if (self.player_curvature - self.track_curvature).abs() >= 0.8 {
            self.speed -= 2.5 * elapsed_time;
        }

        if self.speed < 0.0 {
            self.speed = 0.0;
        }
        if self.speed > 1.0 {
            self.speed = 1.0;
        }

        self.car_distance += (70.0 * self.speed) * elapsed_time;

        let mut offset = 0.0;
        let mut track_section = 0;

        while track_section < self.track.len() && offset <= self.car_distance {
            offset += self.track[track_section].1;
            track_section += 1;
        }

        let target_curvature = self.track[track_section - 1].0;
        let curvature_diff = (target_curvature - self.curvature) * elapsed_time * self.speed;
        self.curvature += curvature_diff;

        self.track_curvature += self.curvature * elapsed_time * self.speed;

        // draw road, clip, and grass
        for y in 0..(utils.height / 2) {
            for x in 0..utils.width {
                let perspective = y as f64 / (utils.height as f64 / 2.0);
                let middle = 0.5 + self.curvature * (1.0 - perspective).powf(3.0);
                let mut road_width = 0.1 + perspective * 0.8;
                let clip_width = road_width * 0.15;
                road_width /= 2.0;

                let left_grass = (middle - road_width - clip_width) * utils.width as f64;
                let right_grass = (middle + road_width + clip_width) * utils.width as f64;
                let left_clip = (middle - road_width) * utils.width as f64;
                let right_clip = (middle + road_width) * utils.width as f64;

                let y_flip = (utils.height / 2) + y;

                let grass_arg: f64 = 20.0 * (1.0 - perspective).powf(3.0) + self.car_distance * 0.1;
                let grass_color = if grass_arg.sin() > 0.0 {
                    Color::Green
                } else {
                    Color::DarkGreen
                };

                let clip_arg: f64 = 80.0 * (1.0 - perspective).powf(2.0) + self.car_distance;
                let clip_color = if clip_arg.sin() > 0.0 {
                    Color::Red
                } else {
                    Color::White
                };

                match x as f64 {
                    x if x < left_grass => utils.draw(x as usize, y_flip, '█', grass_color),
                    x if x >= left_grass && x < left_clip => {
                        utils.draw(x as usize, y_flip, '█', clip_color)
                    }
                    x if x >= left_clip && x < right_clip => {
                        utils.draw(x as usize, y_flip, '█', Color::DarkGrey)
                    }
                    x if x >= right_clip && x < right_grass => {
                        utils.draw(x as usize, y_flip, '█', clip_color)
                    }
                    x if x >= right_grass => utils.draw(x as usize, y_flip, '█', grass_color),
                    _ => (),
                }
            }
        }

        // draw car
        self.car_position = self.player_curvature - self.track_curvature;
        let car_x =
            utils.width as f64 / 2.0 + ((utils.width as f64 * self.car_position) / 2.0) - 7.0;
        let car_y = (0.8 * utils.height as f64) as usize;

        utils.draw_string(car_x as usize, car_y + 0, "   ||####||   ", Color::White, true);
        utils.draw_string(car_x as usize, car_y + 1, "      ##      ", Color::White, true);
        utils.draw_string(car_x as usize, car_y + 2, "     ####     ", Color::White, true);
        utils.draw_string(car_x as usize, car_y + 3, "     ####     ", Color::White, true);
        utils.draw_string(car_x as usize, car_y + 4, "|||  ####  |||", Color::White, true);
        utils.draw_string(car_x as usize, car_y + 5, "|||########|||", Color::White, true);
        utils.draw_string(car_x as usize, car_y + 6, "|||  ####  |||", Color::White, true);
    }
}

fn main() -> Result<()> {
    let rules = RacerRules::new();
    let mut game = ConsoleGameEngine::new(60, 96, rules);
    game.construct_console()?;
    game.start(true)?;

    Ok(())
}
