fn main() {
    let subject = "World";
    let  greeting = format!("Hello, {}!", subject);
    println!("{}, {}!", greeting, subject);
     let crash_reason = "Server wanted a nap.";
    panic!("I crashed! {}", crash_reason);
    println!("This will never get run."); 
}