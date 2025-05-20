#![allow(unused_variables, dead_code)]

use std::ops::AddAssign;

#[derive(Clone)]
struct Bike {
    name: String,
    weight: u32,
}

impl Bike {
    fn new(name: String, weight: u32) -> Self {
        let result = Self { name, weight };
        result
    }

    fn print_bike(&self) {
        println!("My bike is a {} and weighs {} kg", self.name, self.weight);
    }

    fn change_weight(&mut self, weight: u32) {
        self.weight = weight;
    }
}

#[derive(Copy, Clone)]
struct Vector2d {
    x: f32,
    y: f32,
}

impl Vector2d {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let my_bike = Bike::new("Trek".to_string(), 10);
    my_bike.print_bike();
    
    let mut my_other_bike = my_bike;
    println!("My bike is a {} and weighs {} kg", my_other_bike.name, my_other_bike.weight);

    let reader = &my_other_bike;
    let reader_2 = &my_other_bike;
    reader.print_bike();
    reader_2.print_bike();
    my_other_bike.weight += 1;
    
    let reader_writer = &mut my_other_bike;
    reader_writer.change_weight(15);
    reader_writer.print_bike();
    my_other_bike.weight += 1;

    drop(my_other_bike);

    let v = Vector2d::new(1.0, 2.0);
    let v2 = v;
    println!("v: {}, v2: {}", v.x, v2.x);

    let v3 = v2.clone();
    println!("v3: {}, v2: {}", v3.x, v2.x);

    let mut numbers = vec![1, 2, 3, 4, 5];
    for n in &mut numbers {
        println!("n: {}", n);
        n.add_assign(10);
    }
    for n in numbers {
        println!("n: {}", n);
    }
}
