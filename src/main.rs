fn main() {
    let subject = "World";
    let greeting = format!("Hello, {}!", subject);
    println!("{}, {}!", greeting, subject);
    let crash_reason = "Server wanted a nap.";
    println!("This will never get run.");
    floats();
    panic!("I crashed! {}", crash_reason);
}
#[allow(dead_code)]
fn floats() {
    let x = 1.1;
    let y = 2.2;

    println!("x times y is {}", x * y)
}
