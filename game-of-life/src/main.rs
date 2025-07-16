use raylib::prelude::*;
use rand::Rng;

const GRID_WIDTH: usize = 150;
const GRID_HEIGHT: usize = 150;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        let buffer_size = width * height;
        Framebuffer {
            width,
            height,
            buffer: vec![0xFF1E1E3C; buffer_size],
            background_color: 0xFF1E1E3C,
            current_color: 0xFFFFFF00,
        }
    }

    fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    fn get_color(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] == self.current_color
        } else {
            false
        }
    }
}

struct GameOfLife {
    grid: Vec<Vec<bool>>,
    next_grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    generation_count: u32,
}

impl GameOfLife {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![false; width]; height];
        let next_grid = vec![vec![false; width]; height];
        
        let mut game = GameOfLife { 
            grid, 
            next_grid, 
            width, 
            height,
            generation_count: 0,
        };
        game.initialize_patterns();
        game
    }

    fn initialize_patterns(&mut self) {
        // Gosper guns
        self.add_gosper_glider_gun(15, 15);
        self.add_gosper_glider_gun(85, 85);
        
        // Gliders
        self.add_glider(5, 5);
        self.add_glider(140, 140);
        self.add_glider(5, 140);
        self.add_glider(140, 5);
        self.add_glider(75, 10);
        self.add_glider(10, 75);
        self.add_glider(140, 75);
        self.add_glider(75, 140);
        
        // Spaceships
        self.add_lightweight_spaceship(55, 15);
        self.add_middleweight_spaceship(15, 80);
        self.add_heavyweight_spaceship(115, 45);
        
        // Oscillators
        self.add_pulsar(45, 45);
        self.add_pulsar(105, 105);
        self.add_pentadecathlon(70, 25);
        self.add_pentadecathlon(25, 105);
        
        // Random areas
        self.add_random_soup(60, 20, 25, 25);
        self.add_random_soup(20, 90, 25, 25);
        self.add_random_soup(95, 60, 25, 25);
        
        // Some blinkers
        self.add_blinker(30, 30);
        self.add_blinker(120, 120);
        self.add_blinker(30, 120);
        self.add_blinker(120, 30);
        
        // Static patterns
        self.add_block(3, 3);
        self.add_beehive(145, 145);
    }

    fn add_block(&mut self, x: usize, y: usize) {
        let pattern = [
            [true, true],
            [true, true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_beehive(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, true, true, false],
            [true, false, false, true],
            [false, true, true, false]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_blinker(&mut self, x: usize, y: usize) {
        let pattern = [
            [true],
            [true],
            [true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_toad(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, true, true, true],
            [true, true, true, false]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_beacon(&mut self, x: usize, y: usize) {
        let pattern = [
            [true, true, false, false],
            [true, true, false, false],
            [false, false, true, true],
            [false, false, true, true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_pulsar(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, false, true, true, true, false, false, false, true, true, true, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, false],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [false, false, true, true, true, false, false, false, true, true, true, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, true, true, true, false, false, false, true, true, true, false, false],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [true, false, false, false, false, true, false, true, false, false, false, false, true],
            [false, false, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, true, true, true, false, false, false, true, true, true, false, false]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_glider(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, true, false],
            [false, false, true],
            [true, true, true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_lightweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [
            [true, false, false, true, false],
            [false, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_middleweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, false, false, true, false, false],
            [true, false, false, false, false, true],
            [false, false, false, false, false, true],
            [true, false, false, false, true, false],
            [false, true, true, true, true, true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_heavyweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, false, false, true, true, false, false],
            [true, false, false, false, false, false, true],
            [false, false, false, false, false, false, true],
            [true, false, false, false, false, true, false],
            [false, true, true, true, true, true, true]
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_pentadecathlon(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, false, true, false, false, false, false, true, false, false],
            [true, true, false, true, true, true, true, false, true, true],
            [false, false, true, false, false, false, false, true, false, false],
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_gosper_glider_gun(&mut self, x: usize, y: usize) {
        let pattern = [
            [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true],
            [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true],
            [true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            [true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        ];
        self.place_pattern(x, y, &pattern);
    }

    fn add_random_soup(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        for dy in 0..height {
            for dx in 0..width {
                if x + dx < self.width && y + dy < self.height {
                    self.grid[y + dy][x + dx] = rng.gen_bool(0.35);
                }
            }
        }
    }

    fn place_pattern<const W: usize, const H: usize>(&mut self, x: usize, y: usize, pattern: &[[bool; W]; H]) {
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if x + dx < self.width && y + dy < self.height {
                    self.grid[y + dy][x + dx] = cell;
                }
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = ((x as i32 + dx + self.width as i32) % self.width as i32) as usize;
                let ny = ((y as i32 + dy + self.height as i32) % self.height as i32) as usize;
                
                if self.grid[ny][nx] {
                    count += 1;
                }
            }
        }
        
        count
    }

    fn is_active(&self) -> bool {
        let mut live_count = 0;
        
        for row in &self.grid {
            for &cell in row {
                if cell { live_count += 1; }
            }
        }
        
        if live_count < 60 {
            return false;
        }
        
        if self.generation_count > 500 && live_count < 120 {
            return false;
        }
        
        true
    }

    fn reset(&mut self) {
        if !self.is_active() {
            for row in self.grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = false;
                }
            }
            
            let mut rng = rand::thread_rng();
            
            self.add_gosper_glider_gun(rng.gen_range(10..35), rng.gen_range(10..35));
            self.add_gosper_glider_gun(rng.gen_range(100..135), rng.gen_range(100..135));
            
            for _ in 0..6 {
                let x = rng.gen_range(10..140);
                let y = rng.gen_range(10..140);
                self.add_glider(x, y);
            }
            
            for _ in 0..3 {
                let x = rng.gen_range(15..120);
                let y = rng.gen_range(15..120);
                match rng.gen_range(0..3) {
                    0 => self.add_lightweight_spaceship(x, y),
                    1 => self.add_middleweight_spaceship(x, y),
                    _ => self.add_heavyweight_spaceship(x, y),
                }
            }
            
            for _ in 0..4 {
                let x = rng.gen_range(15..110);
                let y = rng.gen_range(15..110);
                self.add_random_soup(x, y, 20, 20);
            }
            
            self.generation_count = 0;
        }
    }

    fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let is_alive = self.grid[y][x];
                
                self.next_grid[y][x] = match (is_alive, neighbors) {
                    (true, n) if n < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, n) if n > 3 => false,
                    (false, 3) => true,
                    (state, _) => state,
                };
            }
        }
        
        std::mem::swap(&mut self.grid, &mut self.next_grid);
        self.generation_count += 1;
        
        self.reset();
    }
}

fn render(framebuffer: &mut Framebuffer, game: &GameOfLife) {
    for y in 0..game.height {
        for x in 0..game.width {
            if game.grid[y][x] {
                framebuffer.set_current_color(0xFFFFFF00);
                framebuffer.point(x, y);
            } else {
                framebuffer.set_current_color(0xFF1E1E3C);
                framebuffer.point(x, y);
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life - Milton Polanco 23471")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);
    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);
    let mut frame_count = 0;

    while !rl.window_should_close() {
        if frame_count % 8 == 0 {
            game.update();
        }
        frame_count += 1;

        framebuffer.clear();
        render(&mut framebuffer, &game);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        let scale_x = WINDOW_WIDTH as f32 / framebuffer.width as f32;
        let scale_y = WINDOW_HEIGHT as f32 / framebuffer.height as f32;
        
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                let pixel = framebuffer.buffer[y * framebuffer.width + x];
                let color = if pixel == 0xFFFFFF00 {
                    Color::YELLOW
                } else {
                    Color::new(30, 30, 60, 255)
                };
                
                let screen_x = (x as f32 * scale_x) as i32;
                let screen_y = (y as f32 * scale_y) as i32;
                let cell_width = scale_x.ceil() as i32;
                let cell_height = scale_y.ceil() as i32;
                
                d.draw_rectangle(screen_x, screen_y, cell_width, cell_height, color);
            }
        }
    }
}
