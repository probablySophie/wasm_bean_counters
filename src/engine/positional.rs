
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Positional2D(pub f64, pub f64);

impl std::ops::AddAssign for Positional2D
{
	fn add_assign(&mut self, rhs: Self) {*self = Self (self.0 + rhs.0, self.1 + rhs.1);}
}


/* This is probably major overkill, but it was fun to write ¯\_(ツ)_/¯ */
macro_rules! lazy_impl_ops {
    ($who:ident, $big:ident, $small:ident, $maths:block) => {
        impl std::ops::$big for $who
        {
        	type Output = Self;
        	fn $small(self, rhs: Self) -> Self::Output
        	{
        		return $maths(self, rhs)
        	}
        }
    };
	($who:ident, $big:ident, $small:ident, $type:ident, $maths:block) => {	
        impl std::ops::$big<$type> for $who
        {
        	type Output = Self;
        	fn $small(self, rhs: $type) -> Self::Output
        	{
        		return $maths(self, rhs)
        	}
        }
    };
}
lazy_impl_ops!(Positional2D, Div, div, { |a:Positional2D, b:Positional2D| 
	{Positional2D (a.0 / b.0, a.1 / b.1)} });
lazy_impl_ops!(Positional2D, Sub, sub, { |a:Positional2D, b:Positional2D| 
	{Positional2D (a.0 - b.0, a.1 - b.1)} });
lazy_impl_ops!(Positional2D, Add, add, { |a:Positional2D, b:Positional2D| 
	{Positional2D (a.0 + b.0, a.1 + b.1)} });
lazy_impl_ops!(Positional2D, Mul, mul, { |a:Positional2D, b:Positional2D| 
	{Positional2D (a.0 * b.0, a.1 * b.1)} });

lazy_impl_ops!(Positional2D, Mul, mul, f64, { |a:Positional2D, b:f64| 
	{Positional2D (a.0 * b, a.1 * b)} });
lazy_impl_ops!(Positional2D, Div, div, f64, { |a:Positional2D, b:f64| 
	{Positional2D (a.0 / b, a.1 / b)} });


