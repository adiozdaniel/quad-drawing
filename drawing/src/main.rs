#![allow(future_incompatible)]

mod geometrical_shapes;

use geometrical_shapes::*;
use rand::Rng;
use raster::{Color, Image};

fn main() {
    let width = 800;
    let height = 600;
    let mut image = Image::blank(width, height);

    let mut rng = rand::thread_rng();

    // Draw 100 shapes with a balanced distribution.
    for _ in 0..100 {
        let color = random_color(&mut rng);
        let shape_choice = rng.gen_range(0..10);
        match shape_choice {
            0 => Point::random(width, height).draw(&mut image, &color),
            1 => Line::random(width, height).draw(&mut image, &color),
            2 => {
                let p1 = Point::random(width, height);
                let p2 = Point::random(width, height);
                Rectangle::new(&p1, &p2).draw(&mut image, &color);
            }
            3 => {
                let a = Point::random(width, height);
                let b = Point::random(width, height);
                let c = Point::random(width, height);
                Triangle::new(&a, &b, &c).draw(&mut image, &color);
            }
            // More chance for circles.
            4 | 5 | 6 | 7 | 8 | 9 => {
                Circle::random(width, height).draw(&mut image, &color);
            }
            _ => {}
        }
    }

    raster::save(&image, "image.png").unwrap();
}

// Implement Displayable for Image, cloning the color as needed.
impl geometrical_shapes::Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: &Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let _ = self.set_pixel(x, y, color.clone());
        }
    }
}

fn random_color<R: rand::Rng>(rng: &mut R) -> Color {
    Color::rgb(
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    )
}
