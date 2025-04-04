use raster::{Color, Image};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, image: &mut Image, color: &Color);
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: &Color);
}

// Point
#[derive(Clone)]
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
    fn draw(&self, image: &mut Image, color: &Color) {
        image.display(self.x, self.y, color);
    }
}

// Line
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line {
            start: p1.clone(),
            end: p2.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Line::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image, color: &Color) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };
        let mut err = dx - dy;

        let (mut x, mut y) = (self.start.x, self.start.y);

        loop {
            image.display(x, y, color);
            if x == self.end.x && y == self.end.y {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}

// Rectangle
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            top_left: Point::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            bottom_right: Point::new(p1.x.max(p2.x), p1.y.max(p2.y)),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image, color: &Color) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);

        Line::new(&self.top_left, &top_right).draw(image, color);
        Line::new(&top_right, &self.bottom_right).draw(image, color);
        Line::new(&self.bottom_right, &bottom_left).draw(image, color);
        Line::new(&bottom_left, &self.top_left).draw(image, color);
    }
}

// Triangle
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image, color: &Color) {
        Line::new(&self.a, &self.b).draw(image, color);
        Line::new(&self.b, &self.c).draw(image, color);
        Line::new(&self.c, &self.a).draw(image, color);
    }
}

// Circle
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        // Increased the radius range for bigger circles.
        Circle {
            center: Point::random(width, height),
            radius: rng.gen_range(50..150),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image, color: &Color) {
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            let cx = self.center.x;
            let cy = self.center.y;
            for (dx, dy) in &[
                (x, y),
                (y, x),
                (-y, x),
                (-x, y),
                (-x, -y),
                (-y, -x),
                (y, -x),
                (x, -y),
            ] {
                image.display(cx + dx, cy + dy, color);
            }
            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x + 1);
            }
        }
    }
}
