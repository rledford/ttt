pub const V_WIDTH: i32 = 480;
pub const V_HEIGHT: i32 = 360;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    None,
    Loading,
    Playing,
    Depot,
    GameOver,
}
