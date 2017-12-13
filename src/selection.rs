/// A day/part selection.
#[derive(Copy, Clone)]
pub struct Selection {
    day: u8,
    part: u8,
}

impl Selection {
    /// Constructor.
    pub fn new(day: u8, part: u8) -> Self {
        Selection {
            day,
            part,
        }
    }

    /// Get the day.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Get the part.
    pub fn part(&self) -> u8 {
        self.part
    }
}
