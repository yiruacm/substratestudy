use std::f32::consts::PI;

trait GraphicArea {
    fn area(self: &Self) -> f32;
}

fn area<T: GraphicArea>(shape: &T) -> f32 {
    shape.area()
}

struct Rec {
    l: f32,
    w: f32,
}

impl GraphicArea for Rec {
    fn area(self: &Self) -> f32 {
        self.l * self.w
    }
}

struct Circle(f32);

impl GraphicArea for Circle {
    fn area(self: &Self) -> f32 {
        PI * self.0* self.0
    }
}

struct Tri {
    a: f32,
    h: f32,
}

impl GraphicArea for Tri {
    fn area(self: &Self) -> f32 {
        0.5 * self.a * self.h
    }
}
fn main() {
    let r = Rec {l:4.0,w:3.0};
    let c = Circle(3.0);
    let t = Tri { a: 10.0, h: 5.0};
    println!("{:?}",area(&r));
    println!("{:?}",area(&c));
    println!("{:?}",area(&t));
}
