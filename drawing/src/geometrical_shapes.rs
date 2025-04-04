use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&mut self, canva: &mut Image);
    fn color() -> Color;
}

pub trait Displayable {
    fn display(canva: &mut Image, x: i32, y: i32, color: Color);
}

#[derive(Debug, Eq, PartialEq)]

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x_coord: i32, y_coord: i32) -> Self {
        Self {
            x: x_coord,
            y: y_coord,
        }
    }

    pub fn clone(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y,
        };
    }

    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        return Point { x: rng.gen_range(0, width+1), 
            y: rng.gen_range(0, height)};
    }

}

impl Drawable for Point{
    fn color() -> Color {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(1, 255);
        let g = rng.gen_range(1, 255);
        let b = rng.gen_range(1, 255);

        return Color::rgb(r, g, b);
    }

    fn draw(&mut self, canva: &mut Image){
        Image::display(canva, self.x, self.y, Point::color())
    }
}
