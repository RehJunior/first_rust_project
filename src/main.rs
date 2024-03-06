fn main() {
    let subject = "World";
    let greeting = format!("Hello, {}!", subject);
    println!("{}, {}!", greeting, subject);
    println!("This will never get run.");
    floats();
    let answer = multiply(1.1, 2.2);
    println!("1.1 * 2.2 = {} ", answer);
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
fn multiply(x: f64, y: f64) -> f64 {
    return x * y;
}
