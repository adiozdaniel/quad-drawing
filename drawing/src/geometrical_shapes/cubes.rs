use super::{Drawable, Point, Line};
use crate::geometrical_shapes::Displayable;
use raster::{Color, Image};
use rand::Rng;

#[derive(Debug)]
pub struct Cubes {
    cubes: Vec<(Point, i32, Color)>,
}

impl Cubes {
    pub fn new(position: &Point, size: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();
        
        let base_color = Color::rgb(
            rng.gen_range(150..255),
            rng.gen_range(150..255),
            rng.gen_range(150..255),
        );

        for _ in 0..rng.gen_range(3..6) {
            let offset_x = rng.gen_range(-size..size);
            let offset_y = rng.gen_range(-size..size);
            cubes.push((
                Point::new(position.x + offset_x, position.y + offset_y),
                rng.gen_range(size/2..size*3/2).max(20),
                base_color.clone()
            ));
        }
        
        Cubes { cubes }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();
        
        let base_color = Color::rgb(
            rng.gen_range(150..255),
            rng.gen_range(150..255),
            rng.gen_range(150..255),
        );

        for _ in 0..rng.gen_range(3..6) {
            cubes.push((
                Point::random(width, height),
                rng.gen_range(30..80),
                base_color.clone()
            ));
        }
        
        Cubes { cubes }
    }

    fn get_isometric_projection(center: &Point, size: i32) -> [Point; 8] {
        let x = center.x;
        let y = center.y;
        let s = size;

        [
            Point::new(x - s, y - s/2),
            Point::new(x + s, y - s/2),
            Point::new(x - s, y + s/2),
            Point::new(x + s, y + s/2),
            Point::new(x - s/2, y - s),
            Point::new(x + s/2, y - s),
            Point::new(x - s/2, y),
            Point::new(x + s/2, y)
        ]
    }
}

impl Drawable for Cubes {
    fn draw(&self, image: &mut Image) {
        for (center, size, color) in &self.cubes {
            let vertices = Cubes::get_isometric_projection(center, *size);
            
            let edges = [
                (0, 1), (1, 3), (3, 2), (2, 0),
                (4, 5), (5, 7), (7, 6), (6, 4),
                (0, 4), (1, 5), (2, 6), (3, 7)
            ];

            for (i, j) in edges.iter() {
                let start = &vertices[*i];
                let end = &vertices[*j];
                
                for offset in -1..=1 {
                    let adjusted_start = Point::new(start.x + offset, start.y + offset);
                    let adjusted_end = Point::new(end.x + offset, end.y + offset);
                    Line::from_points(&adjusted_start, &adjusted_end).draw(image);
                }
            }
        }
    }

    fn color(&self) -> Color {
        self.cubes[0].2.clone()
    }
}
