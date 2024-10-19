use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;

use crate::world::World;
use crate::canvas_handler;

#[wasm_bindgen]
pub struct Game
{
	world: World,
	time: u32,
}

const DELTA_MILLISECONDS: u32 = 75;

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
		}
	}

	pub fn update(&mut self, deltatime: u32)
	{
		self.time += deltatime;
		
		// TODO: Update every time enough deltatime has passed that we get 60 FPS
		/*
			if self.elapsed_milliseconds >= TICK_MILLISECONDS {
            	self.elapsed_milliseconds = 0;
            	self.world.tick();
        	}
		*/
	}

	pub fn draw(&mut self, context: &CanvasRenderingContext2d)
	{
		// Clear the canvas
		context.clear_rect(0., 0., self.world.get_width().into(), self.world.get_height().into());

		self.world.draw(context);	
	}

	pub fn mouse_moved(&mut self, x: i32, y: i32)
	{
		self.world.mouse_moved(x, y);
	}
}
