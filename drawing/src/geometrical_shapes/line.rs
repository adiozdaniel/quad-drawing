use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Line {
    start: Point,
    end: Point,
    thickness: i32,
}

impl Line {
    pub fn new(_p1: &Point, _p2: &Point) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let mut rng = rand::thread_rng();
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);

        let start = Point::random(1000, 1000);
        let (dx, dy) = if count == 0 {
            (rng.gen_range(300..500), rng.gen_range(300..500))
        } else {
            (rng.gen_range(50..120), rng.gen_range(50..120))
        };

        let end = Point::new(
            (start.x + dx).min(999),
            (start.y + dy).min(999),
        );

        Line {
            start,
            end,
            thickness: rng.gen_range(2..5),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
            thickness: rand::thread_rng().gen_range(2..5),
        }
    }

    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        Line {
            start: p1.clone(),
            end: p2.clone(),
            thickness: rand::thread_rng().gen_range(2..5),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = Color::rgb(
            rand::thread_rng().gen_range(50..200),
            rand::thread_rng().gen_range(50..200),
            rand::thread_rng().gen_range(50..200),
        );
        
        for t in 0..self.thickness {
            let offset = t - self.thickness/2;
            let dx = (self.end.x - self.start.x).abs();
            let dy = (self.end.y - self.start.y).abs();
            let sx = if self.start.x < self.end.x { 1 } else { -1 };
            let sy = if self.start.y < self.end.y { 1 } else { -1 };
            let mut err = dx - dy;
            let (mut x, mut y) = (self.start.x, self.start.y);
            
            loop {
                if dx > dy {
                    image.display(x, y + offset, color.clone());
                } else {
                    image.display(x + offset, y, color.clone());
                }
                
                if x == self.end.x && y == self.end.y { break; }
                let e2 = 2 * err;
                if e2 > -dy { err -= dy; x += sx; }
                if e2 < dx { err += dx; y += sy; }
            }
        }
    }
}
