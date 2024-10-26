
/* With immense thanks to the Algorithm Archive for their Verlet Integration examples */
/* https://www.algorithm-archive.org */

#[path="positional.rs"] mod positional;
pub use crate::engine::physics::positional::Positional2D;

#[derive(Default)]
pub struct PhysicsObject
{
	position: Positional2D,
	previous_position: Positional2D,
	velocity: Positional2D,
	acceleration: Positional2D,
	pub mass: f64,
}
const GRAVITY: Positional2D = Positional2D(0., 10.);

impl PhysicsObject
{
	/// Update the physics object.  This uses Verlet integration, so its not the cheapest of physics-s, but it should be decent for our purposes
	pub fn update(&mut self, timestep: f64)
	{		
		// We need to know where we were
		self.previous_position = self.position;

		// This is a velocity Verlet Integration
		// It's not the fastest, but neither am I, so...
		self.position += self.velocity * timestep +  self.acceleration * 0.5 * timestep * timestep;
		self.velocity += self.acceleration * timestep;

		// And reset acceleration
		self.acceleration = GRAVITY;
		// Though doing it this way does inherently assume an air friction/drag of 0
	}

	pub fn new(x: f64, y: f64, acceleration_x: f64, acceleration_y: f64, mass: f64) -> Self
	{
		Self
		{
			position:          Positional2D(x, y),
			previous_position: Positional2D(x, y),
			velocity:          Positional2D(0., 0.),
			acceleration:      Positional2D(acceleration_x, acceleration_y),
			mass
		}
	}

	pub fn x(&self) -> f64 { self.position.0 }
	pub fn y(&self) -> f64 { self.position.1 }
	
	pub fn set_x(&mut self, new: f64) { 
		self.position.0 = new; 
		self.previous_position.0 = new + self.velocity.0;}
	
	pub fn set_y(&mut self, new: f64) { 
		self.position.1 = new; 
		self.previous_position.1 = new + self.velocity.1;}
	
	pub fn accelerate(&mut self, x: f64, y: f64)
	{
		self.acceleration += Positional2D (x, y);
	}
}
