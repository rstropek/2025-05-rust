#![allow(unused_variables, dead_code)]

#[derive(PartialEq, Eq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

enum HotelRoom {
    Vacant,
    Occupied(String),
    UnderMaintenance(String),
}

impl HotelRoom {
    //...
}

// impl Display for HotelRoom {}

fn main() {
    let c: Color = Color::Red;

    match c {
        Color::Red | Color::Yellow => println!("Red or Yellow"),
        Color::Blue => println!("Blue"),
        _ => println!("{:?}", c),
    };

    let message = match c {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
        _ => "Unknown",
    };

    println!("{}", message);

    let n = 5;

    match n {
        1..4 => println!("1..4"),
        5..10 => println!("5..10"),
        _ => println!("Unknown"),
    };

    let hotel_room = HotelRoom::Vacant;
    let hotel_room = HotelRoom::Occupied("John Doe".to_string());
    let hotel_room = HotelRoom::UnderMaintenance("Repairing".to_string());

    match &hotel_room {
        HotelRoom::Vacant => println!("Vacant"),
        HotelRoom::Occupied(name) => println!("Occupied by {}", name),
        HotelRoom::UnderMaintenance(reason) => println!("Under Maintenance: {}", reason),
    };

    if let HotelRoom::Occupied(name) = hotel_room {
        println!("Occupied by {}", name);
    }

    let numbers: [i32; 0] = [];
    let first_number = numbers.iter().next().unwrap();

    let number = "42".parse::<i32>();
    match number {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {}", e),
    };
}
