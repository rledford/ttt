use raylib::prelude::*;

use crate::{
    core::{
        game_state::GameState,
        game_zone,
        states::{loading::LoadingState, playing::PlayingState},
    },
    entities::obstacle::ObstacleType,
    rendering::renderer,
};

pub struct GameStateManager {
    current_state: GameState,
    previous_state: Option<GameState>,

    loading_state: Option<LoadingState>,
    playing_state: Option<PlayingState>,
}

impl Default for GameStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            current_state: GameState::None,
            previous_state: None,
            loading_state: None,
            playing_state: None,
        }
    }

    pub fn update(&mut self, dt: f32, rl: &RaylibHandle) {
        match self.current_state {
            GameState::None => {
                self.transition_to(GameState::Loading);
            }
            GameState::Loading => {
                if let Some(state) = &mut self.loading_state {
                    let result = crate::core::states::loading::update(state, dt);

                    if let Some(new_state) = result {
                        self.transition_to(new_state);
                    }
                }
            }
            GameState::Playing => {
                if let Some(state) = &mut self.playing_state {
                    let gt = rl.get_time();
                    let input = &crate::systems::input::read_input(rl);
                    let result = crate::core::states::playing::update(state, input, dt, gt);

                    if let Some(new_state) = result {
                        self.transition_to(new_state);
                    }
                }
            }
            GameState::GameOver => {}
        }
    }

    pub fn transition_to(&mut self, new_state: GameState) {
        self.on_exit(self.current_state);
        self.previous_state = Some(self.current_state);
        self.current_state = new_state;
        self.on_enter(self.current_state);
    }

    pub fn render(
        &self,
        renderer: &mut renderer::Renderer,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
    ) {
        renderer.prepare(rl);

        {
            let mut d = rl.begin_texture_mode(rt, &mut renderer.target);

            match self.current_state {
                GameState::None => {}
                GameState::Loading => {
                    d.clear_background(Color::RAYWHITE);
                }
                GameState::Playing => {
                    d.clear_background(Color::BLACK);

                    if let Some(state) = &self.playing_state {
                        let zone_meta =
                            game_zone::get_zone_meta_for_distance(state.distance_traveled);

                        {
                            let color = if state.destruction_window_timer > 0.0 {
                                Color::ORANGE
                            } else {
                                Color::GREEN
                            };
                            d.draw_rectangle_rec(state.player.collider(), color);
                        }

                        for o in &state.obstacles {
                            let color = match o.kind {
                                ObstacleType::LaunchDebris => Color::DARKORANGE,
                                ObstacleType::FlockOfBirds => Color::STEELBLUE,
                                ObstacleType::WeatherBallon => Color::YELLOW,
                                ObstacleType::Drone => Color::DARKORCHID,
                            };

                            let collider = o.collider();

                            d.draw_rectangle_rec(collider, color);

                            if o.is_in_destruction_range {
                                d.draw_rectangle_lines_ex(collider, 2.0, Color::WHITE);
                            }
                        }

                        for i in 0..state.max_hp {
                            if state.hp > i {
                                d.draw_circle(i * 10 + 5, 10, 5.0, Color::RED);
                            } else {
                                d.draw_circle(i * 10 + 5, 10, 5.0, Color::BLUE);
                            }
                        }

                        d.draw_text(
                            &format!("Speed: {:.0}", state.speed),
                            0,
                            20,
                            16,
                            Color::GREEN,
                        );
                        d.draw_text(
                            &format!("Dist: {:.0}", state.distance_traveled),
                            0,
                            40,
                            16,
                            Color::GREEN,
                        );
                        d.draw_text(
                            &format!("Zone: {}", zone_meta.name),
                            0,
                            60,
                            16,
                            Color::GREEN,
                        );
                        d.draw_text(&format!("Heat: {:.0}", state.heat), 0, 80, 16, Color::GREEN);
                    }
                }
                GameState::GameOver => {}
            }
        }

        renderer.show(rl, rt);
    }

    fn on_enter(&mut self, state: GameState) {
        match state {
            GameState::None => {}
            GameState::Loading => {
                println!("Enter loading");
                if self.loading_state.is_none() {
                    self.loading_state = Some(LoadingState { timer: 0.0 });
                }
            }
            GameState::Playing => {
                println!("Enter playing");
                if self.playing_state.is_none() {
                    self.playing_state = Some(PlayingState::new());
                }
            }
            GameState::GameOver => {}
        }
    }

    fn on_exit(&mut self, state: GameState) {
        match state {
            GameState::None => {}
            GameState::Loading => {
                println!("Exit lodaing");
            }
            GameState::Playing => {
                println!("Exit playing");
            }
            GameState::GameOver => {}
        }
    }
}
