pub fn array() {
    let mut years: [i32; 3] = [1995, 2000, 2005];

    let _first_year = years[0];
    let [_, _second_year, _third_year] = years;

    years[2] = 2010;
    // years[x] = 2010; ////! this will panic if its out of bounds!

    for year in years.iter() {
        println!("Next year: {}", year + 1)
    }
}
// Arrays können geloopt werden 
// Tuples und Structs nicht 
// Array elemente müssen den selben type haben und eine fixe Länge