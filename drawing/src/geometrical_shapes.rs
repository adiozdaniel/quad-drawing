use raster::{Color, Image};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        Color::rgb(0, 0, 0) // default: black
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point
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
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0) // red
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
            start: Point::new(p1.x, p1.y),
            end: Point::new(p2.x, p2.y),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Line::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };
        let mut err = dx - dy;

        let (mut x, mut y) = (self.start.x, self.start.y);

        loop {
            image.display(x, y, self.color());
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

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0) // green
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
    fn draw(&self, image: &mut Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);

        Line::new(&self.top_left, &top_right).draw(image);
        Line::new(&top_right, &self.bottom_right).draw(image);
        Line::new(&self.bottom_right, &bottom_left).draw(image);
        Line::new(&bottom_left, &self.top_left).draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255) // blue
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
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            c: Point::new(c.x, c.y),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(&self.a, &self.b).draw(image);
        Line::new(&self.b, &self.c).draw(image);
        Line::new(&self.c, &self.a).draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 0) // yellow
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
            center: Point::new(center.x, center.y),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Circle {
            center: Point::random(width, height),
            radius: rng.gen_range(5..50),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
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
                image.display(cx + dx, cy + dy, self.color());
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

    fn color(&self) -> Color {
        Color::rgb(255, 105, 180) // pink
    }
}
