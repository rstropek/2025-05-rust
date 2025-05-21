#![allow(unused_variables, dead_code)]

use std::{cell::RefCell, rc::Rc};

struct MyPreciousRing {
    engraving: String,
}

fn heat(ring: &mut MyPreciousRing) {
    // The inscription becomes readable...
    ring.engraving =
        "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul."
            .to_string();
}

impl MyPreciousRing {
    fn forge() -> Self {
        Self {
            engraving: "One ring to rule them all".to_string(),
        }
    }
}

#[derive(Clone)]
struct A {}

trait MyClone {
    fn clone(&self) -> Self;
}

impl MyClone for A {
    fn clone(&self) -> Self {
        Self {}
    }
}

fn main() {
    {
        let saurons_ring = Rc::new(MyPreciousRing::forge());
        println!(
            "Sauron's ring has {} strong references",
            Rc::strong_count(&saurons_ring)
        );
        let frodos_ring = Rc::clone(&saurons_ring);
        println!("{:p} == {:p}", saurons_ring.as_ref(), frodos_ring.as_ref());
        println!("{}", frodos_ring.engraving);
        println!(
            "Sauron's ring has {} strong references",
            Rc::strong_count(&saurons_ring)
        );

        drop(frodos_ring);
        println!(
            "Sauron's ring has {} strong references",
            Rc::strong_count(&saurons_ring)
        );
        println!("{}", saurons_ring.engraving);
    }
    let a = A {};
    let b = Clone::clone(&a);
    let b = <A as Clone>::clone(&a);
    let b = MyClone::clone(&a);

    {
        // Inner mutability pattern
        let saurons_ring = Rc::new(RefCell::new(MyPreciousRing::forge()));
        let frodos_ring = Rc::clone(&saurons_ring);
        {
            let frodos_ring_borrowed = &frodos_ring.borrow();
        }
        
        heat(&mut saurons_ring.borrow_mut());
        println!("{}", saurons_ring.borrow().engraving);
    }
}
