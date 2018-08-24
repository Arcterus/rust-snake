extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

use graphics::color::{self, BLACK, WHITE};
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use piston_window::*;
use rand::random;
use sdl2_window::Sdl2Window;

const UPDATE_PERIOD: f64 = 0.083; // ~12 updates/s
const WINDOW_HEIGHT: u32 = 480;
const WINDOW_WIDTH: u32 = 640;
const BLOCK_SIZE: u32 = 10;
const ROWS: usize = WINDOW_HEIGHT as usize / BLOCK_SIZE as usize;
const COLS: usize = WINDOW_WIDTH as usize / BLOCK_SIZE as usize;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    grid: [[Option<Block>; COLS]; ROWS],
    snake: Vec<Block>,
    new_block: Block,
}

#[derive(PartialEq, Clone, Copy)]
struct Block {
    loc: Location,
}

#[derive(PartialEq, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

struct Overlay {
    blocks: Vec<Block>,
}

struct App {
    grid: Grid,
    started: bool,
    game_over: bool,
    overlay: Overlay,
    direction: Direction,
    want_direction: Option<Direction>,
    elapsed: f64,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Grid {
    fn new() -> Self {
        let mut grid = Grid {
            grid: [[None; COLS]; ROWS],
            snake: vec![Block::new(Location::new(COLS / 2, ROWS / 2))],
            new_block: Block::new(Location::new(0, 0)),
        };
        grid.add_block();
        grid
    }

    fn insert(&mut self, block: Block) {
        let (x, y) = (block.loc.x, block.loc.y);
        if !self.valid(x, y) {
            return;
        }

        if self.grid[y][x].is_none() || self.grid[y][x].unwrap() != block {
            self.grid[y][x] = Some(block);
        }
    }

    fn add_block(&mut self) {
        let x = random::<usize>() % COLS;
        let y = random::<usize>() % ROWS;
        let block = Block::new(Location::new(x, y));
        if self.contains(&block) {
            self.add_block();
        } else {
            self.insert(block);
            self.new_block = block;
        }
    }

    fn move_snake(&mut self, direction: Direction) {
        let mut blocks = vec![];
        let mut oldblock = self.head().in_direction(self, direction);
        self.grid[oldblock.loc.y][oldblock.loc.x] = Some(oldblock);
        for &block in self.snake.iter().rev() {
            blocks.push(oldblock);
            oldblock = block;
        }
        self.grid[oldblock.loc.y][oldblock.loc.x] = None;
        blocks.reverse();
        self.snake = blocks;
    }

    fn add_to_snake(&mut self, block: Block) {
        self.snake.push(block);
    }

    fn head(&self) -> Block {
        *self.snake.last().unwrap()
    }

    fn contains(&self, block: &Block) -> bool {
        if self.valid(block.loc.x, block.loc.y) {
            self.grid[block.loc.y][block.loc.x].is_some()
        } else {
            false
        }
    }

    fn valid(&self, x: usize, y: usize) -> bool {
        self.valid_x(x) && self.valid_y(y)
    }

    fn valid_x(&self, x: usize) -> bool {
        x < self.grid[0].len()
    }

    fn valid_y(&self, y: usize) -> bool {
        y < self.grid.len()
    }

    fn render<G: Graphics>(&self, c: &Context, g: &mut G, color: Color) {
        for block in &self.snake {
            block.render(c, g, color);
        }
        self.new_block.render(c, g, color);
    }
}

impl Block {
    fn new(loc: Location) -> Self {
        Block { loc }
    }

    fn in_direction(&self, grid: &Grid, dir: Direction) -> Self {
        let gridv = &grid.grid;
        let (x, y) = match dir {
            Direction::Up => (self.loc.x, self.loc.y.wrapping_sub(1)),
            Direction::Down => (self.loc.x, self.loc.y.wrapping_add(1)),
            Direction::Left => (self.loc.x.wrapping_sub(1), self.loc.y),
            Direction::Right => (self.loc.x.wrapping_add(1), self.loc.y),
        };
        Block::new(if grid.valid_x(x) {
            if grid.valid_y(y) {
                Location::new(x, y)
            } else if y == gridv.len() {
                Location::new(x, 0)
            } else {
                Location::new(x, gridv.len() - 1)
            }
        } else if x == gridv[0].len() {
            Location::new(0, y)
        } else {
            Location::new(gridv[0].len() - 1, y)
        })
    }

    fn render<G: Graphics>(&self, c: &Context, gl: &mut G, color: Color) {
        let x = (self.loc.x * BLOCK_SIZE as usize) as f64;
        let y = (self.loc.y * BLOCK_SIZE as usize) as f64;

        Rectangle::new(color).draw(
            [x, y, f64::from(BLOCK_SIZE), f64::from(BLOCK_SIZE)],
            &DrawState::default(),
            c.transform,
            gl,
        );
    }
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        assert!(x <= COLS);
        assert!(y <= ROWS);
        Location { x, y }
    }
}

impl Overlay {
    fn new() -> Self {
        Overlay { blocks: vec![] }.populate()
    }

    fn populate(mut self) -> Self {
        let block_xys = [
            (3,40), (3,41), (5,40), (5,41), (5,42), (5,43), (5,44), (5,45), (6,40),
            (6,43), (7,40), (7,43), (8,41), (8,42), (8,44), (8,45), (10,40), (10,41),
            (14,40), (15,40), (16,40), (16,41), (16,42), (16,43), (16,44), (16,45),
            (17,40), (18,12), (18,13), (18,14), (18,15), (18,16), (18,40), (19,11),
            (19,17), (20,11), (20,17), (20,41), (20,42), (20,43), (20,44), (21,11),
            (21,15), (21,17), (21,40), (21,45), (22,12), (22,15), (22,16), (22,40),
            (22,45), (23,22), (23,23), (23,24), (23,25), (23,26), (23,41), (23,42),
            (23,43), (23,44), (24,12), (24,13), (24,14), (24,15), (24,16), (24,17),
            (24,21), (24,27), (25,11), (25,15), (25,21), (25,27), (26,11), (26,15),
            (26,21), (26,27), (27,11), (27,15), (27,22), (27,23), (27,24), (27,25),
            (27,26), (27,40), (27,41), (27,42), (27,43), (27,44), (27,45), (28,12),
            (28,13), (28,14), (28,15), (28,16), (28,17), (28,40), (28,43), (29,21),
            (29,22), (29,23), (29,40), (29,43), (30,11), (30,12), (30,13), (30,14),
            (30,15), (30,16), (30,17), (30,24), (30,25), (30,26), (30,41), (30,42),
            (30,44), (30,45), (31,12), (31,27), (32,13), (32,24), (32,25), (32,26),
            (32,40), (32,41), (32,42), (32,43), (32,44), (32,45), (33,12), (33,21),
            (33,22), (33,23), (33,40), (33,43), (33,45), (34,11), (34,12), (34,13),
            (34,14), (34,15), (34,16), (34,17), (34,40), (34,45), (35,21), (35,22),
            (35,23), (35,24), (35,25), (35,26), (35,27), (36,11), (36,12), (36,13),
            (36,14), (36,15), (36,16), (36,17), (36,21), (36,24), (36,27), (36,40),
            (36,41), (36,42), (36,45), (37,11), (37,14), (37,17), (37,21), (37,27),
            (37,40), (37,42), (37,43), (37,45), (38,11), (38,17), (38,21), (38,27),
            (38,40), (38,43), (38,44), (38,45), (39,11), (39,17), (40,21), (40,22),
            (40,23), (40,24), (40,25), (40,26), (40,27), (40,40), (41,21), (41,24),
            (41,40), (42,21), (42,24), (42,40), (42,41), (42,42), (42,43), (42,44),
            (42,45), (43,22), (43,23), (43,25), (43,26), (43,27), (43,40), (44,40),
            (46,41), (46,42), (46,43), (46,44), (46,45), (47,40), (47,43), (48,40),
            (48,43), (49,41), (49,42), (49,43), (49,44), (49,45), (51,40), (51,41),
            (51,42), (51,43), (51,44), (51,45), (52,40), (52,43), (53,40), (53,43),
            (54,41), (54,42), (54,44), (54,45), (56,40), (57,40), (58,40), (58,41),
            (58,42), (58,43), (58,44), (58,45), (59,40), (60,40),
        ];

        for &(x, y) in block_xys.iter() {
            self.blocks.push(Block::new(Location::new(x, y)));
        }

        self
    }

    fn render<G: Graphics>(&self, c: &Context, g: &mut G, color: Color) {
        for block in &self.blocks {
            block.render(c, g, color);
        }
    }
}

impl App {
    fn new() -> Self {
        App {
            grid: Grid::new(),
            started: true,
            game_over: false,
            overlay: Overlay::new(),
            direction: Direction::Up,
            want_direction: None,
            elapsed: std::f64::MAX,
        }
    }

    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |ref c, gl| {
            gl.clear_color(WHITE);

            if self.game_over {
                self.grid.render(c, gl, color::grey(0.7));
                self.overlay.render(c, gl, BLACK);
            } else {
                self.grid.render(c, gl, BLACK);
            }
        });
    }

    fn update(&mut self, args: UpdateArgs) {
        self.elapsed += args.dt;
        if self.elapsed < UPDATE_PERIOD {
            return;
        }

        self.elapsed = 0.0;
        if !self.game_over && self.started {
            self.update_logic();
        }
    }

    fn key_release(&mut self, key: Key) {
        match key {
            Key::R => {
                self.grid = Grid::new();
                self.started = true;
                self.game_over = false;
                self.direction = Direction::Up;
                self.want_direction = None;
                self.elapsed = std::f64::MAX;
            }
            Key::P | Key::Return => self.started = !self.started,
            Key::Up | Key::W => self.want_direction = Some(Direction::Up),
            Key::Down | Key::S => self.want_direction = Some(Direction::Down),
            Key::Left | Key::A => self.want_direction = Some(Direction::Left),
            Key::Right | Key::D => self.want_direction = Some(Direction::Right),
            _ => {}
        }
    }

    fn update_direction(&mut self) {
        if let Some(dir) = self.want_direction {
            if self.direction != dir.opposite() {
                self.direction = dir;
            }
            self.want_direction = None;
        }
    }

    fn update_logic(&mut self) {
        self.update_direction();

        let near_head = self.grid.head().in_direction(&self.grid, self.direction);
        if near_head == self.grid.new_block {
            let block = self.grid.new_block;
            self.grid.add_to_snake(block);
            self.grid.add_block();
        } else if self.grid.contains(&near_head) {
            self.game_over = true;
        } else {
            self.grid.move_snake(self.direction);
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    assert!(WINDOW_WIDTH % BLOCK_SIZE == 0);
    assert!(WINDOW_HEIGHT % BLOCK_SIZE == 0);

    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .fullscreen(false)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = App::new();
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(args) = e.button_args() {
            if let ButtonArgs {
                state: ButtonState::Release,
                button: Button::Keyboard(key),
                ..
            } = args
            {
                app.key_release(key)
            }
        }
    }
}
