pub fn tuples() {
    let point: (i64, i64, i64) = (0, 0, 0);

    let x = point.0;
    let y = point.1;
    let z = point.2;
    // Destructuring
    let (x, y, z) = point;
    let (x, y, _) = point;
    let (x, _, _) = point;
    // Mutability
    let mut point: (i64, i64, i64) = (0, 0, 0);
    point.0 = 17;
    point.1 = 42;
    point.2 = 90;
    // Unit
    let unit: () = ();
    #[allow(dead_code)]
    fn unit() {}
    #[allow(dead_code)]
    fn unit2() -> () {}
    let println_return_val: () = println!("Hi!");
}
