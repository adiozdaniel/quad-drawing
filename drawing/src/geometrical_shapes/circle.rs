use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;

pub struct Circle {
    circles: Vec<(Point, i32, Color)>,
}

impl Circle {
    pub fn new(_center: &Point, _radius: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();
        
        for _ in 0..rng.gen_range(1..2) {
            circles.push((
                Point::random(900, 900),
                rng.gen_range(50..150),
                Color::rgb(
                    rng.gen_range(50..200),
                    rng.gen_range(50..200),
                    rng.gen_range(50..200),
                )
            ));
        }
        
        Circle { circles }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();
        
        for _ in 0..rng.gen_range(1..2) {
            circles.push((
                Point::random(width, height),
                rng.gen_range(50..150),
                Color::rgb(
                    rng.gen_range(50..200),
                    rng.gen_range(50..200),
                    rng.gen_range(50..200),
                )
            ));
        }
        
        Circle { circles }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        for (center, radius, color) in &self.circles {
            let mut x = *radius;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                for (dx, dy) in &[
                    (x, y), (y, x), (-y, x), (-x, y),
                    (-x, -y), (-y, -x), (y, -x), (x, -y)
                ] {
                    image.display(center.x + dx, center.y + dy, color.clone());
                }
                
                y += 1;
                err += 1 + 2*y;
                if 2*(err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2*x;
                }
            }
        }
    }

    fn color(&self) -> Color {
        self.circles[0].2.clone()
    }
}
