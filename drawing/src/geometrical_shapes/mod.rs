mod point;
mod line;
mod rectangle;
mod triangle;
mod circle;
mod pentagon;
mod cubes;

pub use self::point::Point;
pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::triangle::Triangle;
pub use self::circle::Circle;
pub use self::pentagon::Pentagon;
pub use self::cubes::Cubes;

use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

// Remove the Displayable trait implementation for Image from here
// Keep just the trait declaration
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
