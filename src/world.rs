use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::player;

#[wasm_bindgen]
pub struct World
{
	width: i32,
	height: i32,

	player: player::Player,
}

#[wasm_bindgen]
impl World {

	pub fn new (width: i32, height: i32) -> World
	{
		World
		{
			width,
			height,
			player: player::Player::new(),
		}
	}

	pub fn get_width  (&self) -> i32 {self.width }
	pub fn get_height (&self) -> i32 {self.height}

	pub fn mouse_moved(&mut self, x: i32, y:i32)
	{
		self.player.update_location(x, y);
	}

	pub fn draw(&mut self, context: &CanvasRenderingContext2d)
	{
		context.begin_path();
		context.rect(
			self.player.get_x(), 
			self.player.get_y(self.height.into()),
			self.player.width,
			self.player.height);
		context.stroke();
	}
}
