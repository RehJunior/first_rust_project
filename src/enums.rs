enum Color {                                 // type
    Green,                                   // variant
    Yellow,                                  // variant
    Red,                                     // variant
    Custom { red: u8, green: u8, blue: u8 }, // structs example
    Custom2(u8, u8, u8),                     // tuples example
}
pub fn examples() {
    let _go = Color::Green;
    let _stop = Color::Red;
    let _slow_down = Color::Yellow;

    let _go: Color = Color::Green;
    let _stop: Color = Color::Red;
    let _slow_down: Color = Color::Yellow;
    let _purple: Color = Color::Custom {
        red: 100,
        green: 0,
        blue: 250,
    };
    let _purple2: Color = Color::Custom2(100, 0, 250);
    let current_color = Color::Yellow;

   let _color_str =  match current_color {
        Color::Green => {
            println!("It was green!");
        }
        Color::Yellow => {
            println!("It was yellow!");
        }
        Color::Red => {
            println!("It was red!")
        }
        Color::Custom { red, green, blue } => {
            println!("{} {} {}", red, green, blue);
        }
        Color::Custom2(red, green, blue) => {
            println!("{} {} {}", red, green, blue);
        }
    };
}
