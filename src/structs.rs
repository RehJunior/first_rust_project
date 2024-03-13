pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
// Ein weg um den point zu returnen 
pub fn _new_point(x: i64, y: i64, z: i64) -> Point {
    Point { x: x, y: y, z: z }
}
// Ein anderer kürzerer Weg zum returnen 
pub fn new_point1(x: i64, y: i64, z: i64) -> Point {
    Point { x, y, z }
}
pub fn _destruction() {
    let point = Point { x: 1, y: 2, z: 3 };

    let Point { x: x1, y, z: z1 } = point;
    println!("x1: {}, y: {}, z1: {}", x1, y, z1); // Nutze x1 und z1

    let Point { x: x2, y: _, z: z2 } = point;
    println!("x2: {}, z2: {}", x2, z2); // Nutze x2 und z2

    let Point { x: x3, z: z3, .. } = point;
    println!("x3: {}, z3: {}", x3, z3); // Nutze x3 und z3

    let Point { x: x4, .. } = point;
    println!("x4: {}", x4); // Nutze x4
}

pub fn _mutability() {
    let mut point = Point { x: 1, y: 2, z: 3 };
    point.x = 5;
}
