use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::{engine::ColliderObject, world};

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
	pub x: f64,
	pub y: f64,
	pub width: f64,
	pub height: f64,
	state: State,
	bags_carried: u8,
	pub collider: ColliderObject,
	bonus_lives: u8,
}

#[wasm_bindgen]
impl Player
{
	fn ouch(&mut self)
	{
		if self.bonus_lives > 0
		{ 
			self.bonus_lives -= 1;
			return // TODO: Invuln frames
		}
		// Else
		self.state = State::Dead;
	}
	fn beans(&mut self)
	{
		self.bags_carried += 1;
		// TODO: Update the image we're using
		// TODO: Die if too many beans
	}
	
	#[allow(clippy::match_same_arms)]
	pub fn message(&mut self, message: world::Message) -> Option<world::Message>
	{
		match message
		{
			world::Message::NothingToSay => {}//
			world::Message::PlayerHit    => { self.ouch() },
			world::Message::BeansCaught  => { self.beans() },
			world::Message::ExtraLife    => { self.bonus_lives += 1; },
		    world::Message::GameOver     => { /* Do nothing */ },
		}
				
		if self.state == State::Dead
		{
			return Some(world::Message::GameOver)
		}

		None // return None
	}
	
	#[allow(clippy::cast_possible_truncation)]
	// make a new player
	pub fn new (world_height: f64) -> Player
	{
		let my_height = 200.;
		let my_width = 100.;

		Player
		{
			x: 0.,
			y: -50. + world_height - my_height,
			height: my_height,
			width:  my_width,
			state: State::Alive,
			bags_carried: 0,
			collider: ColliderObject::new_rectangle(my_width, 30.),
			bonus_lives: 0,
		}
	}

	#[allow(clippy::cast_possible_truncation)]
	/// Take the mouse's location & update the player's location
	/// And offset to use the mouse as the player's center
	pub fn update_location (&mut self, x: i32, y: i32)
	{
		self.x = f64::from(x) - self.width/2.;
		//self.y = y;
		let _ = y;

		self.collider.set_pos(self.x, self.y);
	}

	pub fn draw(&self, context: &CanvasRenderingContext2d)
	{
		context.begin_path();
		context.rect(
			self.x, 
			self.y,
			self.width,
			self.height
		);

		context.set_fill_style_str("red");
		context.fill_rect(
			self.collider.x(),
			self.collider.y(),
			self.collider.width,
			self.collider.height
		);
		context.set_fill_style_str( "black" );
		
		context.stroke();
		
	}
}
