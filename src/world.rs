use crate::{js, levels::Level, objects::ObjectType, DEBUG};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::{objects::{Debris, FlyingObject}, player};


#[wasm_bindgen]
pub struct World
{
	width: f64,
	height: f64,

	player: player::Player,
	objects: Vec<FlyingObject>,
	debris: Vec<Debris>,

	level: Level,
	score_string: String,
	debug_string: String,
	score: i32,
	string_update: f64
}

#[wasm_bindgen]
impl World {

	pub fn new (width: i32, height: i32) -> World
	{
		console_log!("Creating new world!");
		World
		{
			width: width.into(),
			height: height.into(),
			player: player::Player::new(height.into()),
			objects: Vec::new(),
			debris: Vec::new(),
			
			score_string: String::new(),
			debug_string: String::new(),
			string_update: 10.,
			
			level: Level::new(1),
			score: 0,
		}
	}

	pub fn update(&mut self, deltatime: f64)
	{
		self.string_update -= deltatime;
		if self.string_update < 0. 
		{
			self.string_update = 10.;
			self.debug_string = String::from("Debris: ") + &self.debris.len().to_string()
				+ "\nFlying Items: " + &self.objects.len().to_string()
				+ "\nSpawn Timer: " + &self.level.timer.to_string();	
			self.score_string = String::from("Lives: ∞   ") 
				+ "Truck: " + &self.level.level_num.to_string() + "   "
				+ "Score: " + &self.score.to_string();
		}
		
		// We don't update the player in here
		self.level.update(deltatime, &mut self.objects, self.width, self.height);

		// Collision!
		let mut to_remove: Vec<usize> = Vec::new();
		for (i, flying) in self.objects.iter().enumerate()
		{
			if self.player.collider.check(&flying.collider)
			{
				match flying.kind 
				{
					    ObjectType::Coffee => {
					    	self.player.add_beans();
					    	to_remove.push(i);
					    },
					    ObjectType::Anvil => todo!(),
					    ObjectType::Fluffy => todo!(),
					    ObjectType::Flower => todo!(),
					    ObjectType::Life => todo!(),
				}
			}
		}
		for i in &to_remove {self.objects.remove(*i);}

		// If throwables are below a certain Y value, kill them and spawn a debris object where they were
		let mut to_remove: Vec<usize> = Vec::new();
		self.objects.iter_mut().enumerate().for_each(|(i, flying)| 
			{
				flying.update(deltatime);
				if flying.y() > self.height - 50.
				{
					to_remove.push(i);
					self.debris.push(Debris::new(flying));
				}
			});
		for i in &to_remove {self.objects.remove(*i);}
		
		// Kill the debris when they're no longer needed
		let mut to_remove: Vec<usize> = Vec::new();
		self.debris.iter_mut().enumerate().for_each(|(i, debris)| 
			{
				debris.life -= deltatime;
				if debris.life < 0. {to_remove.push(i - to_remove.len());}
			});
		for i in &to_remove {self.debris.remove(*i);}

		// Lives
		// Truck
		// Score
	}

	pub fn get_width  (&self) -> f64 { self.width }
	pub fn get_height (&self) -> f64 { self.height }

	pub fn mouse_moved(&mut self, x: i32, y:i32)
	{
		self.player.update_location(x, y);
	}
	pub fn mouse_pressed(&mut self, x: i32, y: i32)
	{
		self.objects.push( FlyingObject::new(
			f64::from(x), 
			f64::from(y), 
			-200.,
			-300.,
			crate::objects::ObjectType::Coffee) );
		//console_log!("{} flying objects", self.objects.len());
		console_log!("{}, {}", x, y);
	}

	pub fn draw(&mut self, context: &CanvasRenderingContext2d)
	{
		// Draw a player rectangle
		self.player.draw(context);

		// Call .draw() on each object
		draw_vec!(self, objects, context);
		draw_vec!(self, debris, context );

		// TODO: A better & larger font
		let _ = context.fill_text(&self.score_string, 10., 20.);

		if DEBUG
		{
			context.set_fill_style_str("grey");
			let _ = context.fill_text(&self.debug_string, 10., 40.);
			context.set_fill_style_str("black");
		}
	}
}

