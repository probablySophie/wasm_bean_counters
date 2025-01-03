use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;

use crate::world::World;
use crate::canvas_handler;

#[wasm_bindgen]
pub struct Game
{
	world: World,
	time: u32,

	elapsed_milliseconds: u32,
}

const DELTA_MILLISECONDS: f64 = 75.;

#[wasm_bindgen]
impl Game
{
	pub fn new() -> Game
	{
		let (width, height) = canvas_handler::setup();
		
		Game
		{
			world: World::new(width, height),
			time: 0,
			elapsed_milliseconds: 0,
		}
	}

	pub fn update(&mut self, deltatime: u32)
	{
		self.time += deltatime;

		self.world.update(f64::from(deltatime) / DELTA_MILLISECONDS);
	}

	/// Draw the game
	pub fn draw(&mut self, context: &CanvasRenderingContext2d)
	{
		// Clear the canvas
		context.clear_rect(0., 0., self.world.get_width(), self.world.get_height());

		self.world.draw(context);
	}

	pub fn mouse_moved(&mut self, x: i32, y: i32)
	{
		self.world.mouse_moved(x, y);
	}
	pub fn mouse_pressed(&mut self, x: i32, y: i32)
	{
		self.world.mouse_pressed(x, y);
	}
}
