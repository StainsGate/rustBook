use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_minute = minutes % 60;
        let mut borrowed_hour = 0;

        if new_minute < 0 {
            new_minute += 60;
            borrowed_hour = -1;
        }

        let add_hour = minutes / 60;
        let mut new_hour = (hours + add_hour) % 24 + borrowed_hour;

        if new_hour < 0 {
            new_hour += 24;
        }

        Clock {
            hours: new_hour,
            minutes: new_minute,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let add_result = self.minutes + minutes;
        let mut hour_change = 0;
        let minutes_result;

        if add_result > 60 {
            hour_change = add_result / 60;
            minutes_result = add_result % 60;
        } else if add_result < 0 {
            hour_change = add_result / 60 - 1;
            minutes_result = (add_result % 60) + 60;
        } else {
            minutes_result = add_result;
        }

        let mut hour_result = self.hours + hour_change;

        if hour_result > 0 {
            hour_result = hour_result % 24;
        } else {
            hour_result = (hour_result % 24) + 24;

            if hour_result == 24 {
                hour_result = 0;
            }
        }

        Clock {
            hours: hour_result,
            minutes: minutes_result,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>02}:{:>02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>02}:{:>02}", self.hours, self.minutes)
    }
}
