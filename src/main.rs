use ttt::{
    core::{
        game_state::{V_HEIGHT, V_WIDTH},
        game_state_manager,
    },
    rendering::renderer,
};

fn main() {
    let (mut rl, rt) = raylib::init().size(V_WIDTH, V_HEIGHT).title("TTT").build();

    rl.set_target_fps(60);

    let mut renderer = renderer::Renderer::new(&mut rl, &rt, V_WIDTH, V_HEIGHT);
    let mut game_state_manager = game_state_manager::GameStateManager::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        game_state_manager.update(dt, &rl);
        game_state_manager.render(&mut renderer, &mut rl, &rt)
    }
}
