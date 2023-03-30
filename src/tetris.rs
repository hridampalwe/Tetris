pub mod tetrimino;
use std::time::SystemTime;

const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];

pub use tetrimino::Tetrimino;
pub struct Tetris {
    pub game_map: Vec<Vec<u8>>,
    pub current_level: u32,
    pub nb_lines: u32,
    pub current_piece: Option<Tetrimino>,
    pub score: u32,
}

impl Tetris {
    pub fn new() -> Tetris {
        let game_map = vec![vec![0; 10]; 16];
        Tetris {
            game_map,
            current_level: 1,
            nb_lines: 0,
            current_piece: None,
            score: 0,
        }
    }
    pub fn is_time_over(&self, timer: &SystemTime) -> bool {
        match timer.elapsed() {
            Ok(time) => {
                let milis = time.as_secs() as u32 * 10000 + time.subsec_nanos() / 1_000_000;
                milis >= LEVEL_TIMES[self.current_level as usize - 1]
            }
            Err(_) => false,
        }
    }
    fn update_score(&mut self, score_to_add: u32) {
        self.score += score_to_add;
    }
    fn increase_line(&mut self) {
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.current_level += 1;
        }
    }
    fn check(&mut self) {
        let mut y = 0;
        let mut score_to_add = 0;
        while y < self.game_map.len() {
            let mut po = true;
            for i in self.game_map[y].iter() {
                if *i == 0 {
                    po = false;
                    break;
                }
            }
            if po == true {
                score_to_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
            score_to_add += 1000;
        }
        self.update_score(score_to_add);
        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0; 10]);
        }
    }
    pub fn make_permanent(&mut self) {
        let mut score_to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y: usize = 0;
            while shift_y < piece.states[piece.current_state as usize].len()
                && shift_y + piece.y < self.game_map.len()
            {
                let mut shift_x = 0;
                while shift_x < piece.states[piece.current_state as usize][shift_y].len()
                    && (shift_x as isize + piece.x)
                        < self.game_map[shift_y + piece.y].len() as isize
                {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[shift_y + piece.y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            score_to_add += self.current_level;
        }
        self.update_score(score_to_add);
        self.check();
        self.current_piece = None;
    }
}
