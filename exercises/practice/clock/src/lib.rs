use std::fmt::Display;

const DAY_MINS: i32 = 1440;
const HOUR_MINS: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::build(hours * HOUR_MINS + minutes)
    }

    fn build(minutes: i32) -> Self {
        let mut minutes = minutes;
        while minutes < 0 {
            minutes += DAY_MINS;
        }

        Clock {
            minutes: minutes % DAY_MINS,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::build(self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = self.minutes;
        write!(f, "{:02}:{:02}", minutes / HOUR_MINS, minutes % HOUR_MINS)
    }
}
