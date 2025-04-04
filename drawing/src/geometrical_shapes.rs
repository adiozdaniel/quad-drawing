use raster::{Color, Image};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

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

pub struct Line {
    start: Point,
    end: Point,
    thickness: i32,
}

impl Line {
       pub fn new(_p1: &Point, _p2: &Point) -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let mut rng = rand::thread_rng();
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);

        let start = Point::random(1000, 1000);
        let (dx, dy) = if count == 0 {
            // First line: long
            (
                rng.gen_range(300..500),
                rng.gen_range(300..500),
            )
        } else {
            // Other lines: short but not tiny
            (
                rng.gen_range(50..120),
                rng.gen_range(50..120),
            )
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

pub struct Rectangle {
    rects: Vec<(Point, Point)>,
}

impl Rectangle {
    pub fn new(_p1: &Point, _p2: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut rects = Vec::new();
        
        // Generate 2-3 random rectangles at different positions
        for _ in 0..rng.gen_range(2..4) {
            let width = rng.gen_range(100..250);
            let height = rng.gen_range(80..180);
            let pos = Point::random(800, 800);
            rects.push((
                pos.clone(),
                Point::new(pos.x + width, pos.y + height)
            ));
        }
        
        Rectangle { rects }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        for (p1, p2) in &self.rects {
            let color = Color::rgb(
                rand::thread_rng().gen_range(100..255),
                rand::thread_rng().gen_range(100..255),
                rand::thread_rng().gen_range(100..255),
            );

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
}

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

pub struct Circle {
    circles: Vec<(Point, i32)>,
}

impl Circle {
    pub fn new(_center: &Point, _radius: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();
        
        // Generate 1 random circle at a random position
        for _ in 0..rng.gen_range(1..2) {
            circles.push((
                Point::random(900, 900),
                rng.gen_range(50..150)
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
                rng.gen_range(50..150)
            ));
        }
        
        Circle { circles }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        for (center, radius) in &self.circles {
            let color = Color::rgb(
                rand::thread_rng().gen_range(50..200),
                rand::thread_rng().gen_range(50..200),
                rand::thread_rng().gen_range(50..200),
            );
            
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
}

#[derive(Debug)]
pub struct Pentagon {
    pub center: Point,
    pub radius: i32,
}

impl Pentagon {
    pub fn new(center: Point, radius: i32) -> Self {
        Pentagon { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Pentagon {
            center: Point::random(width, height),
            radius: rng.gen_range(30..100),
        }
    }

    fn get_vertices(&self) -> Vec<Point> {
        (0..5).map(|i| {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / 5.0;
            Point {
                x: self.center.x + (self.radius as f64 * angle.cos()) as i32,
                y: self.center.y + (self.radius as f64 * angle.sin()) as i32,
            }
        }).collect()
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let vertices = self.get_vertices();
        let color = Color::rgb(
            rand::thread_rng().gen_range(150..255),
            rand::thread_rng().gen_range(150..255),
            rand::thread_rng().gen_range(150..255),
        );

        for i in 0..5 {
            let start = &vertices[i];
            let end = &vertices[(i + 1) % 5];
            
            // Draw with thickness
            for offset in -1..=1 {
                let adjusted_start = Point::new(start.x + offset, start.y);
                let adjusted_end = Point::new(end.x + offset, end.y);
                Line::from_points(&adjusted_start, &adjusted_end).draw(image);
                
                let adjusted_start = Point::new(start.x, start.y + offset);
                let adjusted_end = Point::new(end.x, end.y + offset);
                Line::from_points(&adjusted_start, &adjusted_end).draw(image);
            }
        }
    }
}
