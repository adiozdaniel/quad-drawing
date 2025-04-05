use super::{Drawable, Point, Line};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;

pub struct Triangle {
    tris: Vec<(Point, Point, Point)>,
}

impl Triangle {
    pub fn new(_a: &Point, _b: &Point, _c: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut tris = Vec::new();
        
        for _ in 0..rng.gen_range(2..4) {
            let base = Point::random(800, 800);
            let height = rng.gen_range(80..180);
            let width = rng.gen_range(60..150);
            
            tris.push((
                base.clone(),
                Point::new(base.x + width, base.y),
                Point::new(base.x + width/2, base.y - height)
            ));
        }
        
        Triangle { tris }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        for (a, b, c) in &self.tris {
            let color = Color::rgb(
                rand::thread_rng().gen_range(150..255),
                rand::thread_rng().gen_range(150..255),
                rand::thread_rng().gen_range(150..255),
            );

            for i in 0..3 {
                let offset = i - 1;
                let a_offset = Point::new(a.x + offset, a.y + offset);
                let b_offset = Point::new(b.x + offset, b.y);
                let c_offset = Point::new(c.x, c.y + offset);
                
                Line::from_points(&a_offset, &b_offset).draw(image);
                Line::from_points(&b_offset, &c_offset).draw(image);
                Line::from_points(&c_offset, &a_offset).draw(image);
            }
        }
    }
}
