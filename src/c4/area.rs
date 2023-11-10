use std::f64::consts::PI;

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
}

struct Triangle {
    l: f64,
    h: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.l * self.h / 2.0
    }
}

struct Square {
    l: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.l * self.l
    }
}

fn print_area<T: Area>(a: &T) {
    println!("area is {:?}", a.area())
}

#[cfg(test)]
mod tests {
    use crate::c4::area::{Circle, print_area, Square, Triangle};

    #[test]
    fn test_print_area() {
        let c = Circle { r: 1.0 };
        let t = Triangle { l: 1.0, h: 1.0 };
        let s = Square { l: 1.0 };
        print_area(&c);
        print_area(&t);
        print_area(&s);
    }
}