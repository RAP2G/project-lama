use std::ops::Add;

pub const GRID_X_SIZE: u32 = 40;
pub const GRID_Y_SIZE: u32 = 30;
pub const DOT_SIZE_IN_PXS: u32 = 20;

pub enum GameState {
    Playing,
    Paused,
}

pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct GameContext {
    pub player_pos: Vec<Point>,
    pub player_dir: PlayerDirection,
    pub food: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            player_pos: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_dir: PlayerDirection::Right,
            food: Point(3, 3),
            state: GameState::Paused,
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
        }
    }

    pub fn next_tick(&mut self) {
        match self.state {
            GameState::Playing => self.update_player_pos(),
            GameState::Paused => {}
        }
    }

    fn update_player_pos(&mut self) {
        let head_pos = self.player_pos.first().unwrap();
        let new_head_pos = match self.player_dir {
            PlayerDirection::Up => *head_pos + Point(0, -1),
            PlayerDirection::Down => *head_pos + Point(0, 1),
            PlayerDirection::Left => *head_pos + Point(-1, 0),
            PlayerDirection::Right => *head_pos + Point(1, 0),
        };

        self.player_pos.pop();
        self.player_pos.reverse();
        self.player_pos.push(new_head_pos);
        self.player_pos.reverse();
    }

    pub fn move_up(&mut self) {
        self.player_dir = PlayerDirection::Up;
    }
    pub fn move_down(&mut self) {
        self.player_dir = PlayerDirection::Down;
    }
    pub fn move_left(&mut self) {
        self.player_dir = PlayerDirection::Left;
    }
    pub fn move_right(&mut self) {
        self.player_dir = PlayerDirection::Right;
    }
}
