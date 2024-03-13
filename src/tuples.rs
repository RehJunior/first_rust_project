pub fn tuples() {
    let point: (i64, i64, i64) = (0, 0, 0);

    let _x = point.0;
    let _y = point.1;
    let _z = point.2;
    // Destructuring
    let (_x, _y, _z) = point;
    let (_x, _y, _) = point;
    let (_x, _, _) = point;
    // Mutability
    let mut point: (i64, i64, i64) = (0, 0, 0);
    point.0 = 17;
    point.1 = 42;
    point.2 = 90;
    // Unit
    let _unit: () = ();
    #[allow(dead_code)]
    fn unit() {}
    #[allow(dead_code)]
    fn unit2() -> () {}
    let _println_return_val: () = println!("Hi!");
}
