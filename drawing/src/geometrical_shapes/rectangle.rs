use super::{Drawable, Point, Line};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;

pub struct Rectangle {
    rects: Vec<(Point, Point, Color)>,
}

impl Rectangle {
    pub fn new(_p1: &Point, _p2: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut rects = Vec::new();
        
        let base_color = Color::rgb(
            rng.gen_range(100..255),
            rng.gen_range(100..255),
            rng.gen_range(100..255),
        );

        for _ in 0..rng.gen_range(2..4) {
            let width = rng.gen_range(100..250);
            let height = rng.gen_range(80..180);
            let pos = Point::random(800, 800);
            rects.push((
                pos.clone(),
                Point::new(pos.x + width, pos.y + height),
                base_color.clone()
            ));
        }
        
        Rectangle { rects }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        for (p1, p2, color) in &self.rects {
            for i in 0..3 {  
                let offset = i - 1;
                let top_right = Point::new(p2.x + offset, p1.y);
                let bottom_left = Point::new(p1.x, p2.y + offset);
                
                Line::from_points(p1, &top_right).draw(image);
                Line::from_points(&top_right, p2).draw(image);
                Line::from_points(p2, &bottom_left).draw(image);
                Line::from_points(&bottom_left, p1).draw(image);
            }
        }
    }

    fn color(&self) -> Color {
        self.rects[0].2.clone()
    }
}
