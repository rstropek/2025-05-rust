#![allow(unused_variables, dead_code)]

trait Billable {
    fn get_total_cost_eur(&self) -> f64;

    fn get_total_cost_usd(&self) -> f64 {
        self.get_total_cost_eur() * 1.1
    }
}

// Combining traits
trait DebuggableBillable: Billable + Clone {}

#[derive(Clone, Debug, Default)]
struct ConsultingWork {
    hours: f64,
    rate: f64,
}

// We could...
struct MyF64(f64);

impl Default for MyF64 {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Billable for ConsultingWork {
    fn get_total_cost_eur(&self) -> f64 {
        self.hours * self.rate
    }
}

impl Billable for f64 {
    fn get_total_cost_eur(&self) -> f64 {
        *self
    }
}

fn main() {
    let cw = ConsultingWork {
        hours: 10.0,
        rate: 100.0,
    };

    println!("Total cost in EUR: {}", cw.get_total_cost_eur());
    println!("Total cost in USD: {}", cw.get_total_cost_usd());
}
