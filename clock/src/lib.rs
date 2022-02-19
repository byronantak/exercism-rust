use std::fmt::Debug;
use std::fmt::Display;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Debug for Clock {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, rhs: &Clock) -> bool {
        return self.hours == rhs.hours && self.minutes == rhs.minutes;
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_hours = minutes as f64 / 60_f64;
        let hours_from_minutes = extra_hours.floor() as i32;
        let mut hours = hours.rem_euclid(24);
        hours = hours + hours_from_minutes;
        let minutes = minutes.rem_euclid(60);
        Self {
            hours: hours.rem_euclid(24),
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}
