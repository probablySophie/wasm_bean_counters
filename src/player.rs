use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerState
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
	state: PlayerState,
	bags_carried: u8,
}

#[wasm_bindgen]
impl Player
{
	// make a new player
	pub fn new (world_height: i32) -> Player
	{
		let my_height = 200.;
		
		Player
		{
			x: 0,
			y: (-50) + world_height - (my_height as i32), // TODO: better Y value please
			height: my_height,
			width:  100.,
			state: PlayerState::Alive,
			bags_carried: 0,
		}
	}

	pub fn get_x(&self) -> i32 { self.x }
	pub fn get_y(&self) -> i32 { self.y }

	// take the mouse's location & update the player's location
	pub fn update_location (&mut self, x: i32, y: i32)
	{
		self.x = x;
		//self.y = y;
		let _ = y;
	}
}
