mod arrays;
mod enums;
mod greetings;
mod structs;
mod task1;
mod task2;
mod tuples;
mod type_parameters;
fn main() {
    enums::examples();
    let rustville: task2::City = task2::new_city(123, false);
    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
    let my_point = structs::new_point1(1, 2, 3);
    println!(
        "Point x: {}, y: {}, z: {}",
        my_point.x, my_point.y, my_point.z
    );
    arrays::array();
    tuples::tuples();
    task1::task();
    greetings::hello();
    greetings::goodbye();
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
    cats();
    type_parameters::types();
}

#[allow(dead_code)]
fn floats() {
    let x = 1.1;
    let y = 2.2;

    println!("x times y is {}", x * y)
}
///! This is a Way to reasign a value with mut so that it is mutabil
// fn mutability() {
//     let mut z = 1.1;
//     z = 2.2;
// }
fn multiplyfloats(x: f64, y: f64) -> f64 {
    x * y
}
fn integer() {
    let ninty = 90;
    let negative_five = -5;
    let one_thousand = 1_000;
    let exactly_three = 10 / 3;
    // let this_whill_panic = 5 / 0; ///! KABOOM!
    println!(
        "{}, {}, {}, {}",
        ninty, negative_five, one_thousand, exactly_three
    )
}
// * Integer Sizes damit kann man in den minus bereich
// * i8   8bits  (1B) -127 to 128
// * i16  16bits (2B) -32,768 to 32,767
// * i32  32bits (4B)
// * i64  64bits (8B)
// * i128 128bits (16B)

// * Unsigned Integers bedeutet keine minus bereiche
// * u8 0-255
// * u16 0-65,535
// * u32 0-4,294,967,295
// * u64 0-18,446,744,073,709,551,615
// * u128 0-170,141,183,460,469,231,731,687,303,715,884,105,728
// * char is basically like u32

fn multiply(x: i64, y: u8) -> i64 {
    x * (y as i64)
}
fn divide(x: i32, y: u16) -> f64 {
    x as f64 / y as f64
}
fn cats() {
    let cats = 0;
    let message = if cats > 1_000 {
        "Too many cats!"
    } else if cats > 1 {
        "Multiple cats!"
    } else {
        "Need more cats"
    };

    println!("{}", message);
}
