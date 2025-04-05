use super::{Drawable};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let color = Color::rgb(
            rand::thread_rng().gen_range(50..200),
            rand::thread_rng().gen_range(50..200),
            rand::thread_rng().gen_range(50..200),
        );
        for dx in -1..=1 {
            for dy in -1..=1 {
                image.display(self.x + dx, self.y + dy, color.clone());
            }
        }
    }
}
