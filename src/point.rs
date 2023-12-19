use num::{abs, Signed};

#[derive(Copy, Clone)]
pub struct Point<T>(pub (T, T));

impl<T: Copy + Signed> Point<T> {
    pub fn manhattan(&self, other: &Point<T>) -> T {
        abs(self.0 .0 - other.0 .0) + abs(self.0 .1 - other.0 .1)
    }
}
