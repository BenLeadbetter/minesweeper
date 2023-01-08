use rand::distributions::Distribution;
use itertools::Itertools;

pub struct Minefield(Vec<Vec<Tile>>);

#[derive(Clone)]
pub struct Tile {
    pub ty: Type,    
    pub status: Status,
}

impl std::default::Default for Tile {
    fn default() -> Self {
        Tile {
            ty: Type::Safe(0),
            status: Status::Unknown,
        }
    }
}

pub struct Data(pub Vec<Vec<bool>>);

impl Data {
    pub fn new_random(width: usize, height: usize, mines: usize) -> Data {
        let mut ret = Data(vec![vec![false; height]; width]);
        let mut mines_set = 0;
        let distribution = rand::distributions::Uniform::from(0..width*height);
        let mut rng = rand::thread_rng();
        loop {
            if mines_set == mines {
                break;
            }
            
            let index = distribution.sample(&mut rng);
            let mine =
                &mut ret.0[index % width][index / width];

            if !*mine {
                *mine = true;
                mines_set += 1;
            }
        }
        return ret;
    }
}

#[derive(Clone)]
pub enum Type {
    Mine,
    Safe(u8),
}

#[derive(Clone, Copy)]
pub enum Status {
    Unknown,
    Flagged,
    Revealed,
    Suspicious,
}

impl Minefield {
    pub fn new(data: Data) -> Self {
        let width = data.0.len();
        let height = data.0[0].len();

        assert!(width >= 1);
        assert!(height >= 1);

        let mut ret = Minefield(vec![vec![Default::default(); height]; width]);
        
        for (i, j) in (0..width as i32).cartesian_product(0..height as i32) {
            if data.0[i as usize][j as usize] {
                *ret.index_mut(i, j).unwrap() = Tile {
                    ty: Type::Mine,
                    status: Status::Unknown,
                };
                let mut increment_neighbour = |i: i32, j: i32| {
                    if let Some(&Tile{ ty: Type::Safe(v), status }) = ret.index(i, j) {
                        ret.0[i as usize][j as usize] = Tile {
                            ty: Type::Safe(v + 1),
                            status,
                        };
                    }
                };
                increment_neighbour(i, j - 1);
                increment_neighbour(i + 1, j - 1);
                increment_neighbour(i + 1, j);
                increment_neighbour(i + 1, j + 1);
                increment_neighbour(i, j + 1);
                increment_neighbour(i - 1, j + 1);
                increment_neighbour(i - 1, j);
                increment_neighbour(i - 1, j - 1);
            }
        }

        ret
    }
    
    pub fn width(&self) -> i32 {
        return self.0.len() as i32
    }

    pub fn height(&self) -> i32 {
        return self.0[0].len() as i32
    }
    
    pub fn index(&self, row: i32, col: i32) -> Option<&Tile> {
        if 
            row >= self.width() || 
            col >= self.height() ||
            row < 0 || col < 0 {
            None
        } else {
            Some(&self.0[row as usize][col as usize])
        }
    }

    pub fn index_mut(&mut self, row: i32, col: i32) -> Option<&mut Tile> {
        if 
            row >= self.width() || 
            col >= self.height() ||
            row < 0 || col < 0 {
            None
        } else {
            Some(&mut self.0[row as usize][col as usize])
        }
    }
    
    pub fn reveal(&mut self, row: i32, col: i32) {
        println!("reveal ({}, {})", row, col);
    }

    pub fn mark(&mut self, row: i32, col: i32) {
        println!("mark ({}, {})", row, col);
    }
}