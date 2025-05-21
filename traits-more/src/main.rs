#![allow(unused_variables, dead_code)]

use rand::prelude::*;

trait Billable {
    fn bill(&self) -> f32;
}

// Change to pub mod so it can be explicitly imported
pub mod pointworthy {
    pub trait Pointworthy {
        fn point(&self) -> f32;
    }
    
}

pub mod blanket {
    use crate::{pointworthy::Pointworthy, Billable};

    // Blanket implementation
    impl<T> Pointworthy for T where T: Billable {
        fn point(&self) -> f32 {
            self.bill() / 100.0
        }
    }
}

struct Consulting {
    hours: f32,
    rate: f32,
}

impl Billable for Consulting {
    fn bill(&self) -> f32 {
        self.hours * self.rate
    }
}

struct Material {
    quantity: f32,
    unit_price: f32,
}

impl Billable for Material {
    fn bill(&self) -> f32 {
        self.quantity * self.unit_price
    }
}

fn create_something_billable() -> Box<dyn Billable> {
    if rand::rng().random_bool(0.5) {
        Box::new(Material {
            quantity: 10.0,
            unit_price: 10.0,
        })
    } else {
        Box::new(Consulting {
            hours: 10.0,
            rate: 100.0,
        })
    }
}

fn create_something_else_billable() -> impl Billable {
    Consulting {
        hours: 10.0,
        rate: 100.0,
    }
}

fn print_something_billable(something_billable: &dyn Billable) {
    println!("{}", something_billable.bill());
}

// We could... just because we can ;-)
impl<'a> From<&'a Box<dyn Billable>> for &'a dyn Billable {
    fn from(something_billable: &'a Box<dyn Billable>) -> Self {
        something_billable.as_ref()
    }
}

impl<'a> From<&'a Consulting> for &'a dyn Billable {
    fn from(customer: &'a Consulting) -> Self {
        customer
    }
}

fn main() {
    let something_billable = create_something_billable();
    println!("{}", something_billable.bill());
    print_something_billable((&something_billable).into());
    print_something_billable(something_billable.as_ref());

    let consulting = Consulting {
        hours: 10.0,
        rate: 100.0,
    };
    print_something_billable((&consulting).into());

    let something_else_billable = create_something_else_billable();
    println!("{}", something_else_billable.bill());
    //print_something_billable(&something_else_billable);

    let billables = vec![
        create_something_billable(),
        create_something_billable(),
    ];

    for billable in billables { 
        println!("{}", billable.bill());    
    }

    // Explicitly import the Pointworthy trait to use it
    
    let pw = &Consulting {
        hours: 10.0,
        rate: 100.0,
    };
    
    {
        use crate::pointworthy::Pointworthy;
        println!("{}", pw.point());
    }

    //println!("{}", pw.point());
}
