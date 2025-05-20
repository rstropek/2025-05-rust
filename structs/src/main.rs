#![allow(unused, dead_code, unused_variables)]

mod clock {
    type HourMinute = u8;

    // Tuple struct
    pub struct TupleClock(HourMinute, HourMinute);
    struct Vector2d(f32);

    pub struct Clock {
        hour: HourMinute,
        minutes: HourMinute,
    }

    impl Clock {
        pub fn new(hour: HourMinute, minutes: HourMinute) -> Self {
            Self { hour, minutes }
        }

        pub fn empty() -> Self {
            Self {
                hour: 0,
                minutes: 0,
            }
        }

        pub fn get_hour(&self) -> HourMinute {
            self.hour
        }

        pub fn get_minutes(&self) -> HourMinute {
            self.minutes
        }
    }

    impl Clock {
        pub fn set_hour(&mut self, hour: HourMinute) {
            self.hour = hour;
        }

        pub fn set_hour_2(&self, hour: HourMinute) -> Self {
            Self {
                hour,
                minutes: self.minutes,
            }
        }

        pub fn set_hour_3(&mut self, hour: HourMinute) -> &mut Self {
            self.hour = hour;
            self
        }

        pub fn set_minutes(&mut self, minutes: HourMinute) {
            self.minutes = minutes;
        }

        pub fn set_minutes_2(&self, minutes: HourMinute) -> Self {
            Self {
                hour: self.hour,
                minutes,
            }
        }
    }
}

mod advanced_clock {
    impl crate::clock::Clock {
        pub fn add_minutes(&mut self, minutes: u8) {
            let new_minutes = self.get_minutes() + minutes;
            if new_minutes >= 60 {
                self.set_hour(self.get_hour() + new_minutes / 60);
                self.set_hour(self.get_hour() % 24);
                self.set_minutes(new_minutes % 60);
            } else {
                self.set_minutes(new_minutes);
            }
        }
    }
}

fn main() {
    let hour = 10;
    let mut c = clock::Clock::new(hour, 10);
    c.set_hour(12);
    println!("{}:{}", c.get_hour(), c.get_minutes());

    let mut c2 = clock::Clock::empty().set_hour_2(10).set_minutes_2(20);

    {
        c2.add_minutes(10);
        println!("{}:{}", c2.get_hour(), c2.get_minutes());
    }
}
