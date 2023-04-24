use rand::Rng;

pub struct Tile {
    pub is_wall: bool,
}

pub struct Dungeon {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Dungeon {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile { is_wall: true }; height]; width];

        let mut dungeon = Dungeon { width, height, tiles };
        dungeon.generate_random_walk();

        dungeon
    }

    fn generate_random_walk(&mut self) {
        // ... (same as before)
    }

    pub fn render(&self) {
        // ... (same as before)
    }
}
