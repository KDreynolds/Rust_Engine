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
        let mut rng = rand::thread_rng();
        let mut x = self.width / 2;
        let mut y = self.height / 2;

        for _ in 0..(self.width * self.height) / 4 {
            self.tiles[x][y].is_wall = false;

            let direction = rng.gen_range(0..4);
            match direction {
                0 if x > 0 => x -= 1,
                1 if x < self.width - 1 => x += 1,
                2 if y > 0 => y -= 1,
                3 if y < self.height - 1 => y += 1,
                _ => (),
            }
        }
    }


fn render(&self) {
    for y in 0..self.height {
        for x in 0..self.width {
            let tile_char = if self.tiles[x][y].is_wall { '#' } else { '.' };
            print!("{}", tile_char);
        }
        println!();
    }
}}

