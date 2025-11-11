use raylib::prelude::*;

pub struct Renderer {
    v_width: i32,
    v_height: i32,
    v_ratio: f32,
    screen_rect: Rectangle,
    source_rect: Rectangle,
    scale: f32,

    pub target: RenderTexture2D,
    pub ui_target: RenderTexture2D,
}

impl Renderer {
    pub fn new(rl: &mut RaylibHandle, rt: &RaylibThread, v_width: i32, v_height: i32) -> Self {
        let target = rl
            .load_render_texture(rt, v_width as u32, v_height as u32)
            .unwrap();

        let ui_target = rl
            .load_render_texture(rt, v_width as u32, v_height as u32)
            .unwrap();

        Self {
            v_width,
            v_height,
            v_ratio: v_width as f32 / v_height as f32,
            screen_rect: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            source_rect: Rectangle::new(0.0, 0.0, v_width as f32, -v_height as f32),
            scale: 1.0,
            target,
            ui_target,
        }
    }

    pub fn prepare(&mut self, rl: &mut RaylibHandle) {
        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();
        let window_ratio = window_width as f32 / window_height as f32;

        if window_ratio > self.v_ratio {
            self.scale = window_height as f32 / self.v_height as f32;
            self.screen_rect.width = self.v_width as f32 * self.scale;
            self.screen_rect.height = window_height as f32;
            self.screen_rect.x = (window_width as f32 - self.screen_rect.width) * 0.5;
            self.screen_rect.y = 0.0;
        } else {
            self.scale = window_width as f32 / self.v_width as f32;
            self.screen_rect.width = window_width as f32;
            self.screen_rect.height = self.v_height as f32 * self.scale;
            self.screen_rect.x = 0.0;
            self.screen_rect.y = (window_height as f32 - self.screen_rect.height) * 0.5;
        }
    }

    pub fn show(&self, rl: &mut RaylibHandle, rt: &RaylibThread) {
        let mut d = rl.begin_drawing(rt);
        d.clear_background(Color::BLACK);

        d.draw_texture_pro(
            self.target.texture(),
            self.source_rect,
            self.screen_rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        d.draw_texture_pro(
            self.ui_target.texture(),
            self.source_rect,
            self.screen_rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        let x = self.screen_rect.width as i32 - d.measure_text("88 FPS", 24);
        d.draw_fps(x, 0);
    }
}
