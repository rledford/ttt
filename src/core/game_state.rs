pub const V_WIDTH: i32 = 480;
pub const V_HEIGHT: i32 = 360;

#[derive(Clone, Copy)]
pub enum GameState {
    None,
    Loading,
    Playing,
    GameOver,
}
