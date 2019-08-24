use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn add_hours(&mut self, hours: i32) {
        self.hours += hours;
        self.hours %= 24;

        if self.hours < 0 {
            self.hours = 24 + self.hours;
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock {
            hours: 0,
            minutes: 0,
        };

        clock.add_hours(hours);
        clock.add_minutes(minutes)
    }

    pub fn add_minutes(mut self, minutes: i32) -> Clock {
        self.add_hours(minutes / 60);
        self.minutes += minutes % 60;

        if self.minutes < 0 {
            self.minutes = 60 + self.minutes;
            self.add_hours(-1);
        }

        if self.minutes > 60 {
            self.minutes %= 60;
            self.add_hours(1);
        }

        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}