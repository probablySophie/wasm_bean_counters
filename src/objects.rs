use core::fmt;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::engine::{ColliderObject, PhysicsObject};

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum ObjectType
{
	Coffee, // Bags
	Anvil,
	Fluffy, // He's a fish
	Flower,
	Life    // Extra Life
}
impl fmt::Display for ObjectType
{
	#[allow(clippy::uninlined_format_args)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Debris
{
	kind: ObjectType,
	x: f64,
	y: f64,
	pub life: f64,
}
impl Debris
{
	pub fn draw(&self, context: &CanvasRenderingContext2d)
	{
		let _ = context.fill_text(&(self.kind.to_string() + " debris"), self.x, self.y);
	}
	pub fn new(from: &FlyingObject) -> Self
	{
		Self
		{
			kind: from.kind,
			x: from.x(),
			y: from.y(),
			life: 50.,
		}
	}
}

#[wasm_bindgen]
pub struct FlyingObject
{
	pub kind: ObjectType,
	physics: crate::engine::PhysicsObject,
	pub collider: crate::engine::ColliderObject,
}

impl FlyingObject
{
	pub fn new(x: f64, y: f64, vx: f64, vy: f64, kind: ObjectType) -> Self
	{
		// TODO: Have different widths & heights for the different objects
		// TODO: And have different masses?
		FlyingObject{ kind, 
			physics: PhysicsObject::new(x, y, vx, vy, 5.), 
			collider: ColliderObject::new_rectangle(50., 50.),
		}
	}

	pub fn update(&mut self, deltatime: f64)
	{
		// Update where we are
		self.physics.update(deltatime);
		// Let our collider know where we are :)
		self.collider.set_pos(self.physics.x(), self.physics.y());
	}

	pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d)
	{
		let x = self.physics.x();
		let y = self.physics.y();
		
		// Just draw a rectangle at the moment
		context.begin_path();
		let _ = context.fill_text(&self.kind.to_string(), x, y);
		context.rect(x, y,
			self.collider.width,
			self.collider.height);
		context.stroke();
	}

	pub fn x(&self) -> f64 {self.physics.x()}
	pub fn y(&self) -> f64 {self.physics.y()}
}

