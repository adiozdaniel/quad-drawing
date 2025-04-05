use super::{Drawable, Point, Line};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Pentagon {
    pentagons: Vec<(Point, i32)>,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut pentagons = Vec::new();
        
        for _ in 0..rng.gen_range(3..6) {
            let offset_x = rng.gen_range(-100..100);
            let offset_y = rng.gen_range(-100..100);
            pentagons.push((
                Point::new(center.x + offset_x, center.y + offset_y),
                rng.gen_range(radius-20..radius+20).max(10)
            ));
        }
        
        Pentagon { pentagons }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut pentagons = Vec::new();
        
        for _ in 0..rng.gen_range(3..6) {
            pentagons.push((
                Point::random(width, height),
                rng.gen_range(30..80)
            ));
        }
        
        Pentagon { pentagons }
    }

    fn get_vertices(center: &Point, radius: i32) -> Vec<Point> {
        (0..5).map(|i| {
            let angle = 2.0 * PI * i as f64 / 5.0;
            Point {
                x: center.x + (radius as f64 * angle.cos()) as i32,
                y: center.y + (radius as f64 * angle.sin()) as i32,
            }
        }).collect()
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        for (center, radius) in &self.pentagons {
            let color = Color::rgb(
                rand::thread_rng().gen_range(150..255),
                rand::thread_rng().gen_range(150..255),
                rand::thread_rng().gen_range(150..255),
            );

            let vertices = Pentagon::get_vertices(center, *radius);
            
            for i in 0..5 {
                let start = &vertices[i];
                let end = &vertices[(i + 1) % 5];
                
                for offset in -1..=1 {
                    let adjusted_start = Point::new(start.x + offset, start.y + offset);
                    let adjusted_end = Point::new(end.x + offset, end.y + offset);
                    Line::from_points(&adjusted_start, &adjusted_end).draw(image);
                }
            }
        }
    }
}
