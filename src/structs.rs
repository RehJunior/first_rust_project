pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

pub fn new_point(x: i64, y: i64, z: i64) -> Point {
    Point { x: x, y: y, z: z }
}
pub fn new_point1(x: i64, y: i64, z: i64) -> Point {
    Point { x, y, z }
}
pub fn destruction() {
    let point = Point { x: 1, y: 2, z: 3 };
    let x = point.x;
    let Point { x, y, z } = point;
    let Point{ x, y:_, z} = point;
    let Point{ x, z, ..} = point;
    let Point{ x, ..} = point;
}
pub fn mutability(){
    let mut point = Point{ x:1 , y: 2, z: 3};
    point.x = 5;
}
