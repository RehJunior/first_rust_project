fn main() {
    let subject = "World";
    let greeting = format!("Hello, {}!", subject);
    println!("{}, {}!", greeting, subject);
    println!("This will never get run.");
    floats();
    let answer = multiplyfloats(1.1, 2.2);
    println!("1.1 * 2.2 = {} ", answer);
    integer();
    multiply(-10, 2);
    divide(20, 20);
}
#[allow(dead_code)]
fn floats() {
    let x = 1.1;
    let y = 2.2;

    println!("x times y is {}", x * y)
}
///! This is a Way to reasign a value with mut
// fn mutability() {
//     let mut z = 1.1;
//     z = 2.2;
// }
fn multiplyfloats(x: f64, y: f64) -> f64 {
    return x * y;
}
fn integer() {
    let ninty = 90;
    let negative_five = -5;
    let one_thousand = 1_000;
    let exactly_three = 10 / 3;
    // let this_whill_panic = 5/0; ///! KABOOM!
    println!(
        "{}, {}, {}, {}",
        ninty, negative_five, one_thousand, exactly_three
    )
}
//* Integer Sizes
//* i8   8bits  (1B)
//* i16  16bits (2B)
//* i32  32bits (4B)
//* i64  64bits (8B)
//* i128 128bits (16B)

//* Unsigned Integers
//* u8 0-255
//* u16 0-65,535
//* u32 0-4,294,967,295
//* u64 0-18,446,744,073,709,551,615
//* u128 0-170,141,183,460,469,231,731,687,303,715,884,105,728
//* char is basically like u32

fn multiply(x: i64, y: u8) -> i64 {
    return x * (y as i64);
}
fn divide(x: i32, y: u16) -> f64 {
    return x as f64 / y as f64;
}