extern crate rand;

type Pieces = Vec<Vec<u8>>;
type States = Vec<Pieces>;

pub struct Tetrimino {
    pub states: States,
    pub x: isize,
    pub y: usize,
    pub current_state: u8,
}

impl Tetrimino {
    pub fn test_position(&self, game_map: &[Vec<u8>], x: isize, y: usize, tmpstate: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmpstate][decal_y][decal_x as usize] != 0
                    && (y + decal_y >= game_map.len()
                        || x < 0
                        || x as usize >= game_map[decal_y + y].len()
                        || game_map[y + decal_y][x as usize] != 0)
                {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        let x_pos = [0, 1, -1, 2, -2, -3];
        for i in x_pos.iter() {
            if self.test_position(game_map, self.x + i, self.y, tmp_state as usize) {
                self.current_state = tmp_state;
                self.x += *i;
                break;
            }
        }
    }
    pub fn shift(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, new_x, new_y, self.current_state as usize) {
            self.x = new_x;
            self.y = new_y;
            return true;
        } else {
            return false;
        }
    }
    pub fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.x, self.y, self.current_state as usize)
    }
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

pub struct TetriminoI;
pub struct TetriminoJ;
pub struct TetriminoL;
pub struct TetriminoZ;
pub struct TetriminoS;
pub struct TetriminoO;
pub struct TetriminoT;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 2, 0],
                    vec![0, 0, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 3, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 3, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![4, 4, 0, 0],
                    vec![0, 4, 4, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 4, 0],
                    vec![0, 4, 4, 0],
                    vec![0, 4, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![7, 0, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![7, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![
                vec![0, 6, 6, 0],
                vec![0, 6, 6, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub fn create_new_tetrimino() -> Tetrimino {
    static mut prev: u8 = 7;
    let mut rand_rb = rand::random::<u8>() % 7;
    if unsafe { prev } == rand_rb {
        rand_rb = rand::random::<u8>() % 7;
    }
    unsafe {
        prev = rand_rb;
    }
    match rand_rb {
        0 => TetriminoO::new(),
        1 => TetriminoI::new(),
        2 => TetriminoJ::new(),
        3 => TetriminoL::new(),
        4 => TetriminoZ::new(),
        5 => TetriminoS::new(),
        6 => TetriminoT::new(),
        _ => unreachable!(),
    }
}
