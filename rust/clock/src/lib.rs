use std::fmt;
use std::result::*;


#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

const DAY_IN_M : i32 = 1440;

impl Clock {

    fn normalize(&mut self) -> () {
        let mut min = (self.hours * 60 + self.minutes) % DAY_IN_M;

        while min < 0 {
            min += DAY_IN_M;
        }

        self.hours = min / 60;
        self.minutes = min % 60;
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_clock = Clock { hours, minutes };
        new_clock.normalize();
        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_clock = Clock { hours: self.hours, minutes: self.minutes + minutes };
        new_clock.normalize();
        new_clock
    }

}
