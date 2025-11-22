use raylib::prelude::*;

use crate::{
    core::{
        game_state::GameState,
        game_zone,
        states::{
            depot::DepoState, game_over::GameOverState, loading::LoadingState,
            playing::PlayingState,
        },
    },
    entities::obstacle::ObstacleType,
    rendering::renderer,
};

pub struct GameStateManager {
    current_state: GameState,
    previous_state: Option<GameState>,

    loading_state: Option<LoadingState>,
    playing_state: Option<PlayingState>,
    depot_state: Option<DepoState>,
    game_over_state: Option<GameOverState>,
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
            depot_state: None,
            game_over_state: None,
        }
    }

    pub fn update(&mut self, dt: f32, rl: &RaylibHandle) {
        let input = &crate::systems::input::read_input(rl);

        if input.debug_switch_playing && self.current_state != GameState::Playing {
            println!("debug");
            self.transition_to(GameState::Playing);
        } else if input.debug_switch_depot && self.current_state != GameState::Depot {
            println!("debug");
            self.transition_to(GameState::Depot);
        }

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
            GameState::Depot => {
                if let Some(state) = &mut self.depot_state {
                    let input = &crate::systems::input::read_input(rl);

                    let result = crate::core::states::depot::update(state, input);

                    if let Some(new_state) = result {
                        self.transition_to(new_state);
                    }
                }
            }
            GameState::GameOver => {
                if let Some(state) = &mut self.game_over_state {
                    let input = &crate::systems::input::read_input(rl);

                    let result = crate::core::states::game_over::update(state, input, dt);

                    if let Some(new_state) = result {
                        self.transition_to(new_state);
                    }
                }
            }
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
                    self.render_playing(&mut d);
                }
                GameState::Depot => {
                    self.render_playing(&mut d);
                    self.render_playing_ui(&mut d);
                }
                GameState::GameOver => {
                    self.render_playing(&mut d);
                    self.render_playing_ui(&mut d);
                }
            }
        }

        {
            let mut d = rl.begin_texture_mode(rt, &mut renderer.ui_target);
            match self.current_state {
                GameState::None => {}
                GameState::Loading => {
                    d.clear_background(Color::RAYWHITE);
                }
                GameState::Playing => {
                    self.render_playing_ui(&mut d);
                }
                GameState::Depot => {
                    self.render_depot_ui(&mut d);
                }
                GameState::GameOver => {
                    self.render_game_over_ui(&mut d);
                }
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
            GameState::Depot => {
                println!("Enter depot");
                if self.depot_state.is_none() {
                    self.depot_state = Some(DepoState::new());
                }
            }
            GameState::GameOver => {
                println!("Enter loading");
                if self.game_over_state.is_none() {
                    self.game_over_state = Some(GameOverState { time: 0.0 });
                }
            }
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
            GameState::Depot => {
                println!("Exit depot")
            }
            GameState::GameOver => self.playing_state = Some(PlayingState::new()),
        }
    }

    fn render_playing(&self, d: &mut RaylibTextureMode<RaylibHandle>) {
        d.clear_background(Color::BLACK);

        if let Some(state) = &self.playing_state {
            for p in state.player.left_boost.active_particles() {
                let t = p.age / p.lifetime;
                let scale = p.start_scale * (1.0 - t) + p.end_scale * t;
                d.draw_circle_v(p.position, scale, p.color);
            }

            for p in state.player.right_boost.active_particles() {
                let t = p.age / p.lifetime;
                let scale = p.start_scale * (1.0 - t) + p.end_scale * t;
                d.draw_circle_v(p.position, scale, p.color);
            }

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
        }
    }

    fn render_playing_ui(&self, d: &mut RaylibTextureMode<RaylibHandle>) {
        d.clear_background(Color::BLANK);

        if let Some(state) = &self.playing_state {
            for i in 0..state.max_hp {
                if state.hp > i {
                    d.draw_circle(i * 10 + 5, 10, 5.0, Color::RED);
                } else {
                    d.draw_circle(i * 10 + 5, 10, 5.0, Color::BLUE);
                }
            }

            let zone_meta = game_zone::get_zone_meta_for_distance(state.distance_traveled);

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

    fn render_depot_ui(&self, d: &mut RaylibTextureMode<RaylibHandle>) {
        d.clear_background(Color::BLANK);
        d.draw_text("DEPOT", 0, 0, 48, Color::GREEN);
        if let Some(state) = &self.depot_state {
            d.draw_text(
                &format!("INDEX {}", state.perk_choice_index),
                0,
                200,
                48,
                Color::GREEN,
            );
        }
    }
    fn render_game_over_ui(&self, d: &mut RaylibTextureMode<RaylibHandle>) {
        d.clear_background(Color::BLANK);
        d.draw_text("GAME OVER", 0, 0, 48, Color::RED);
    }
}
