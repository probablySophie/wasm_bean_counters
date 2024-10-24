use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State
{
	Dead,
	Alive,
	Carrying,
}

#[wasm_bindgen]
pub struct Player
{
	pub x: i32,
	pub y: i32,
	pub width: f64,
	pub height: f64,
	state: State,
	bags_carried: u8,
}

#[wasm_bindgen]
impl Player
{
	#[allow(clippy::cast_possible_truncation)]
	// make a new player
	pub fn new () -> Player
	{
		let my_height = 200.;

		Player
		{
			x: 0,
			y: -50,
			height: my_height,
			width:  100.,
			state: State::Alive,
			bags_carried: 0,
		}
	}

	#[allow(clippy::cast_possible_truncation)]
	/// Take the mouse's location & update the player's location
	/// And offset to use the mouse as the player's center
	pub fn update_location (&mut self, x: i32, y: i32)
	{
		self.x = x;
		//self.y = y;
		let _ = y;
	}

	#[allow(clippy::cast_lossless)]
	/// Get the player's X value, but offset by width/2
	pub fn get_x(&self) -> f64 { (self.x as f64) - (self.width/2.) }

	/// Get the player's Y value, but offset by the world.height - self.height
	pub fn get_y(&self, world_height: f64) -> f64 { -50. + world_height - self.height}
}
