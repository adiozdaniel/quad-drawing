use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&mut self, canva: &mut Image);
    fn color() -> Color;
}

pub trait Displayable {
    fn display(canva: &mut Image, x: i32, y: i32, color: Color);
}
