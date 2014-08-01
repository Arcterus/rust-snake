#![feature(globs, phase)]

extern crate collections;
#[phase(plugin, link)] extern crate log;
extern crate getopts;
extern crate graphics;
extern crate piston;

extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::rand::random;
use std::cmp::max;
use std::os;
use graphics::*;
use opengl_graphics::Gl;
use piston::{Game, GameIteratorSettings, GameWindowSettings, KeyPressArgs, RenderArgs, UpdateArgs};
use sdl2_game_window::GameWindowSDL2;

pub static WINDOW_HEIGHT: uint = 480;
pub static WINDOW_WIDTH: uint = 640;

pub static BLOCK_SIZE: uint = 10;  // NOTE: WINDOW_HEIGHT and WINDOW_WIDTH should be divisible by this

#[static_assert]
#[allow(dead_code)]
static _window_width_divisible: bool = WINDOW_WIDTH % BLOCK_SIZE == 0;

#[static_assert]
#[allow(dead_code)]
static _window_height_divisible: bool = WINDOW_HEIGHT % BLOCK_SIZE == 0;

#[deriving(PartialEq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

pub struct Grid {
	grid: Vec<Vec<Option<Block>>>,
	snake: Vec<Block>,
	new_block: Block
}

#[deriving(Clone, PartialEq)]
pub struct Block {
	pub loc: Location
}

#[deriving(Clone, PartialEq)]
pub struct Location {
	pub x: uint,
	pub y: uint
}

pub struct App {
	gl: Gl,
	grid: Grid,
	started: bool,
	game_over: bool,
	direction: Direction,
	updates_since_moved: int, // # of updates since we last moved
	move_threshold: int, // # of updates it takes to move
	variable_snake_speed: bool, // whether to speed up the snake as it grows.
}

impl Grid {
	pub fn new() -> Grid {
		let mut rows: Vec<Vec<Option<Block>>> = vec!();
		rows.reserve(WINDOW_HEIGHT / BLOCK_SIZE);
		for _ in range(0, WINDOW_HEIGHT / BLOCK_SIZE) {
			rows.push(Vec::from_elem(WINDOW_WIDTH / BLOCK_SIZE, None));
		}
		let mut grid = Grid {
			grid: rows,
			snake: vec!(Block::new(Location::new(WINDOW_WIDTH / BLOCK_SIZE / 2,
			                                     WINDOW_HEIGHT / BLOCK_SIZE / 2))),
			new_block: Block::new(Location::new(0, 0))
		};
		grid.add_block();
		grid
	}

	pub fn insert(&mut self, block: Block) {
		let (x, y) = (block.loc.x, block.loc.y);
		if !self.valid(x, y) {
			return;
		}
		let gr_loc = self.grid.get_mut(y).get_mut(x);
		if *gr_loc == None || gr_loc.unwrap() != block {
			*gr_loc = Some(block);
		}
	}

	pub fn remove(&mut self, block: &Block) {
		if self.valid(block.loc.x, block.loc.y) {
			let mut i = 0;
			while i < self.snake.len() {
				if self.snake[i] == *block {
					self.snake.remove(i);
					break;
				}
				i += 1;
			}
			let gr_loc = self.grid.get_mut(block.loc.y).get_mut(block.loc.x);
			*gr_loc = None;
		}
	}

	pub fn add_block(&mut self) {
		let x = random::<uint>() % WINDOW_WIDTH / BLOCK_SIZE;
		let y = random::<uint>() % WINDOW_HEIGHT / BLOCK_SIZE;
		let block = Block::new(Location::new(x, y));
		if self.contains(&block) {
			self.add_block();
		} else {
			self.insert(block);
			self.new_block = block;
		}
	}

	pub fn move_snake(&mut self, direction: Direction) {
		let mut blocks = vec!();
		let mut oldblock = self.head().in_direction(self, direction);
		*self.grid.get_mut(oldblock.loc.y).get_mut(oldblock.loc.x) = Some(oldblock);
		for &block in self.snake.iter().rev() {
			blocks.push(oldblock);
			oldblock = block;
		}
		*self.grid.get_mut(oldblock.loc.y).get_mut(oldblock.loc.x) = None;
		blocks.reverse();
		self.snake = blocks;
	}

	#[inline]
	pub fn add_to_snake(&mut self, block: Block) {
		self.snake.push(block);
	}

	#[inline]
	pub fn head(&self) -> Block {
		self.snake.last().unwrap().clone()
	}

	pub fn contains(&self, block: &Block) -> bool {
		if self.valid(block.loc.x, block.loc.y) {
			self.grid[block.loc.y][block.loc.x].is_some()
		} else {
			false
		}
	}

	#[inline]
	pub fn valid(&self, x: uint, y: uint) -> bool {
		self.valid_x(x) && self.valid_y(y)
	}

	#[inline]
	pub fn valid_x(&self, x: uint) -> bool {
		x < self.grid[0].len()
	}

	#[inline]
	pub fn valid_y(&self, y: uint) -> bool {
		y < self.grid.len()
	}

	#[inline]
	pub fn render(&self, gl: &mut Gl, win_ctx: &Context) {
		for block in self.snake.iter() {
			block.render(gl, win_ctx);
		}
		self.new_block.render(gl, win_ctx);
	}
}

impl Block {
	#[inline]
	pub fn new(loc: Location) -> Block {
		Block {
			loc: loc
		}
	}

	pub fn in_direction(&self, grid: &Grid, dir: Direction) -> Block {
		let gridv = &grid.grid;
		let (x, y) = match dir {
			Up => (self.loc.x, self.loc.y - 1),
			Down => (self.loc.x, self.loc.y + 1),
			Left => (self.loc.x - 1, self.loc.y),
			Right => (self.loc.x + 1, self.loc.y)
		};
		Block::new(
			if grid.valid_x(x) {
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
			}
		)
	}

	#[inline]
	pub fn render(&self, gl: &mut Gl, win_ctx: &Context) {
		win_ctx
		       .rect((self.loc.x * BLOCK_SIZE) as f64, (self.loc.y * BLOCK_SIZE) as f64, BLOCK_SIZE as f64, BLOCK_SIZE as f64)
		       .rgb(0.0, 0.0, 0.0).draw(gl);
	}
}

impl Location {
	#[inline]
	pub fn new(x: uint, y: uint) -> Location {
		assert!(x <= WINDOW_WIDTH / BLOCK_SIZE);
		assert!(y <= WINDOW_HEIGHT / BLOCK_SIZE);
		Location {
			x: x,
			y: y
		}
	}
}

impl App {
	#[inline]
	pub fn new() -> App {
		App {
			gl: Gl::new(),
			grid: Grid::new(),
			started: true,
			game_over: false,
			direction: Up,
			updates_since_moved: 0,
			move_threshold: 40,
			variable_snake_speed: false,
		}
	}
}

impl Game<GameWindowSDL2> for App {
	fn key_press(&mut self, _: &mut GameWindowSDL2, args: &KeyPressArgs) {
		match args.key {
			piston::keyboard::R => {
				self.grid = Grid::new();
				self.started = true;
				self.game_over = false;
			}
			piston::keyboard::P | piston::keyboard::Return => self.started = !self.started,
			piston::keyboard::Up =>
				if self.direction != Down {
					self.direction = Up;
				},
			piston::keyboard::Down =>
				if self.direction != Up {
					self.direction = Down;
				},
			piston::keyboard::Left =>
				if self.direction != Right {
					self.direction = Left;
				},
			piston::keyboard::Right =>
				if self.direction != Left {
					self.direction = Right;
				},
			_ => {}
		}
		debug!("released key: {}", args.key);
	}

	fn update(&mut self, _: &mut GameWindowSDL2, _: &UpdateArgs) {
		if !self.game_over {
			let near_head = self.grid.head().in_direction(&self.grid, self.direction);
			if near_head == self.grid.new_block {
				let block = self.grid.new_block;
				self.grid.add_to_snake(block);
				self.grid.add_block();

				if self.variable_snake_speed {
					// speed up
					self.move_threshold = max(self.move_threshold - 1, 1);
				}
			} else if self.grid.contains(&near_head) {
				self.game_over = true;
			} else {
				self.updates_since_moved += 1;

				if self.updates_since_moved > self.move_threshold {
					self.updates_since_moved -= self.move_threshold;
					self.grid.move_snake(self.direction);
				}
			}
		}
	}

	fn render(&mut self, _: &mut GameWindowSDL2, args: &RenderArgs) {
		if self.game_over {
			// TODO: display game over on screen
		}

		(&mut self.gl).viewport(0, 0, args.width as i32, args.height as i32);
		let ref c = Context::abs(args.width as f64, args.height as f64);
		c.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);

		self.grid.render(&mut self.gl, c);
	}
}

fn main() {
	let mut window = GameWindowSDL2::new(
		GameWindowSettings {
			title: "Snake".to_string(),
			size: [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32],
			fullscreen: false,
			exit_on_esc: true
		}
	);

	let mut app = App::new(); // needs to happen after `window` is created.

	let variable_snake_speed = "variable-snake-speed";
	let args = os::args();
	let opts = [
		getopts::optflag("", variable_snake_speed, "the snake will speed up as it grows")
	];
	match getopts::getopts(args.as_slice(), opts) {
		Ok(m) => {
			if m.opt_present(variable_snake_speed) {
				app.variable_snake_speed = true;
			}
		},
		Err(f) => { fail!(f.to_string()); },
	};

	let game_iter_settings = GameIteratorSettings {
		updates_per_second: 1200,
		max_frames_per_second: 30
	};
	app.run(&mut window, &game_iter_settings);
}

