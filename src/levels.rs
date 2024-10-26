use crate::js;
use wasm_bindgen::prelude::*;
use getrandom::getrandom;

use crate::objects::{FlyingObject, ObjectType};

#[wasm_bindgen]
pub struct Level
{
	pub level_num: i32,
	required_score: i32,
	pub timer: f64,
	random_numbers: [u8; 32],
	random_pointer: usize,
}

impl Level
{
	pub fn new(level_num: i32) -> Self
	{
		let required_score = match level_num {
			1 => {20},
			2 => {25},
			3 => {30},
			4 => {35},
			5 => {40},
			_ => {1000},
		};

		Level
		{
			level_num, 
			required_score,
			timer: 40.,
			random_numbers: [0u8; 32],
			random_pointer: 32,
		 }
	}

	pub fn update(&mut self, deltatime: f64, throwables: &mut Vec<FlyingObject>, world_width: f64, world_height: f64)
	{
		self.timer -= deltatime;

		if self.timer <= 0.
		{
			let x = world_width + 20.;
			//console_log!("Y");
			let y = self.rand_range(100., 300.);
			//console_log!("Velocity X");
			let speed_x = self.rand_range(-450., -250.);
			//console_log!("Velocity Y");
			let speed_y = self.rand_range(-375., -300.);
			
			(*throwables).push( FlyingObject::new( x, y, speed_x, speed_y, ObjectType::Coffee ) );

			//console_log!("Timer");
			self.timer = match self.level_num {
				1 => self.rand_range(10., 30.),
				2 => self.rand_range(10., 25.),
				3 => self.rand_range(10., 20.),
				4 => self.rand_range(10., 17.),
				5 => self.rand_range(10., 15.),
				_ => 0.,
			};
		}
	}

	fn rand(&mut self) -> u8
	{
		if self.random_pointer >= 32
		{
			if getrandom(&mut self.random_numbers).is_err()
			{
				console_log!("Something went wrong while generating a new random number");
			}
			self.random_pointer = 0;
		}
		let rand_u8 = self.random_numbers[self.random_pointer];
		self.random_pointer += 1;

		if rand_u8 == 0 {return rand_u8 + 1}

		rand_u8
	}
	fn rand_range(&mut self, large: f64, small: f64) -> f64
	{
		let rand = self.rand();
		let rand_0_to_1: f64 = f64::from(rand) / 255.;
		
		(rand_0_to_1.abs() * (large.abs() - small.abs())) + small

		//let generated = (rand_0_to_1.abs() * (large.abs() - small.abs())) + small;
		//console_log!("U8: {}. 0-1: {}, num: {}", rand, rand_0_to_1, generated);

		//generated
	}
}

