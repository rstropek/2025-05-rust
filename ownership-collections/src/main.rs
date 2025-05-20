#![allow(unused_variables, dead_code)]

struct Customer {
    name: String,
    age: u8,
}

struct Order<'a> {
    customer: &'a mut Customer,
    amount: u32,
}

struct Vector2d(f32, f32);

impl Vector2d {
    fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

fn get_longer_vec<'a>(v1: &'a Vector2d, v2: &'a Vector2d) -> &'a Vector2d {
    if v1.length() > v2.length() {
        v1
    } else {
        v2
    }
}

fn do_something<'a, 'b>(v1: &'a Vector2d, v2: &'b Vector2d) -> &'a Vector2d {
    // Do something with v1 and v2

    v1
}

fn get_longer_vecs<'a, 'b>(v1: &'a Vector2d, v2: &'a Vector2d, v3: &'b Vector2d, v4: &'b Vector2d) -> (&'a Vector2d, &'b Vector2d) {
    let first_result;
    let second_result;

    if v1.length() > v2.length() {
        first_result = v1;
    } else {
        first_result = v2;
    }

    if v3.length() > v4.length() {
        second_result = v3;
    } else {
        second_result = v4;
    }

    (first_result, second_result)
}

fn main() {
    let c = Customer {
        name: "John".to_string(),
        age: 30,
    };

    let mut customers = Vec::new();
    customers.push(c);

    let c = Customer {
        name: "Jane".to_string(),
        age: 25,
    };

    customers.push(c);

    println!("{}", customers[0].name); // Index trait

    customers[0].name.push('!');
    customers[0].name = "Foo".to_string(); // IndexMut trait
    customers[0].age += 1;

    let c = customers.pop().unwrap();

    let mut iterator = customers.iter_mut();
    let c = iterator.next().unwrap();

    let mut c = Customer {
        name: "John".to_string(),
        age: 30,
    };

    let order = Order {
        customer: &mut c,
        amount: 100,
    };

    order.customer.name = "Foo".to_string();
    
    drop(order);
    drop(c);
    
    //println!("{}", order.customer.name);
    
    let v1 = Vector2d(1.0, 1.0);
    let v_result;
    {
        let v2;
        v2 = Vector2d(2.0, 2.0);
        v_result = get_longer_vec(&v1, &v2);
    }
    
    println!("{}", v_result.length());

}
