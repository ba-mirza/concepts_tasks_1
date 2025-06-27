#[allow(unused)]
#[allow(unused_variables)]

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y
        }        
    }
}

// Позволяет копировать структуру Point без использования clone
impl Copy for Point {}

fn main() {
    let p1 = Point { x: 3.0, y: 2.0 };
    let p2 = p1;

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let p4= Point::default();
    let p5: Point = Default::default();
    println!("p4 (default): {:?}", p4);
    println!("p5 (default): {:?}", p5);
}
