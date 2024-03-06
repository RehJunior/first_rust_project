fn main() {
    let subject = "World";
    let greeting = format!("Hello, {}!", subject);
    println!("{}, {}!", greeting, subject);
    println!("This will never get run.");
    floats();
    mutability();
}
#[allow(dead_code)]
fn floats() {
    let x = 1.1;
    let y = 2.2;

    println!("x times y is {}", x * y)
}

fn mutability() {
    let mut z = 1.1;
    z = 2.2;
    print!("{}", z)
}