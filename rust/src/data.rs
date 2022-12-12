use rand::distributions::Distribution;
use itertools::Itertools;

pub struct Data(Vec<Vec<Tile>>);

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

impl Data {
    pub fn new(width: i32, height: i32, mines: u32) -> Self {
        assert!(width >= 1);
        assert!(height >= 1);
        assert!(mines as i32 <= width * height);

        let mut ret = Data(vec![vec![Default::default(); height as usize]; width as usize]);
        
        let mut mines_set = 0;
        let distribution = rand::distributions::Uniform::from(0..width*height);
        let mut rng = rand::thread_rng();
        loop {
            if mines_set == mines {
                break;
            }
            
            let index = distribution.sample(&mut rng);
            let mut tile = 
                &mut ret.0[(index % width) as usize][(index / width) as usize];

            if let Type::Safe(_) = tile.ty {
                tile.ty = Type::Mine;
                mines_set += 1;
            }
        }
        
        for (i, j) in (0..width).cartesian_product(0..height) {
            if let Type::Mine = ret.index(i, j).unwrap().ty {
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
}