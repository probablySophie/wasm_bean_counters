use super::Positional2D;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Copy)]
#[wasm_bindgen]
pub enum Shape
{
	Rectangle,
	Circle,
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct ColliderObject
{
	pub shape: Shape,
	pub width: f64,
	pub height: f64,
	pub radius: f64,
	position: Positional2D,
}

impl ColliderObject
{
	pub fn new_rectangle(width: f64, height: f64) -> Self
	{
		Self
		{
			shape: Shape::Rectangle,
			width,
			height,
			position: Positional2D(0., 0.),
			radius: ((width/2.)*(width/2.) + (height/2.) + (height/2.)).sqrt()
		}
	}
	pub fn new_circle(radius: f64) -> Self
	{
		Self
		{
			shape: Shape::Circle,
			width: radius*2.,
			height: radius*2.,
			position: Positional2D(0.,0.),
			radius
		}
	}
	
	pub fn check(&mut self, other: &Self) -> bool
	{
		let dx = self.x() - other.x();
		let dy = self.y() - other.y();
		let dist = (dx * dx + dy * dy).sqrt();

		// If they're more than both radiuses away, then they're 100% not colliding
		if dist > self.radius + other.radius
		{
			return false
		}
		// Else
		
		// If they're both circles, then we're happy at this point
		if self.shape == Shape::Circle && other.shape == Shape::Circle
		{
			return true
		}

		// TODO: If one is a circle and the other is a rectangle...

		// Time for a boring X, Y box check

		// X-width check
		if self.x() < other.x() + other.width && self.x() + self.width > other.x()
		{
			// Y-height check
			if self.y() < other.y() + other.height && self.y() + self.height > other.y()
			{
				return true // Wow, they collide!?
			}
		}
		false // if we're here, then nope, they don't collide at all
	}

	// TODO: Work out the direction & distance both colliders would need to move to stop overlapping
	
	pub fn x(&self) -> f64 { self.position.0 }
	pub fn y(&self) -> f64 { self.position.1 }
	pub fn set_x(&mut self, new: f64) {self.position.0 = new}
	pub fn set_y(&mut self, new: f64) {self.position.1 = new}
	pub fn set_pos(&mut self, x: f64, y:f64) {self.position = Positional2D(x, y)}
}
