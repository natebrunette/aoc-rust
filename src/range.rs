#[derive(Clone)]
pub struct Range<T>(pub std::ops::Range<T>);

impl<T: Ord> From<(T, T)> for Range<T> {
    fn from(value: (T, T)) -> Self {
        if value.0 < value.1 {
            Range(value.0..value.1)
        } else {
            Range(value.1..value.0)
        }
    }
}

pub trait RangeExtensions {
    fn intersection(&self, other: &Self) -> Option<Box<Self>>;
    fn subtract(&self, other: &Self) -> Vec<Box<Self>>;
}

impl<T: Ord + Copy + Clone> RangeExtensions for Range<T> {
    fn intersection(&self, other: &Self) -> Option<Box<Self>> {
        let start = self.0.start.max(other.0.start.clone());
        let end = self.0.end.min(other.0.end.clone());

        if start < end {
            Some(Box::new(Range(start..end)))
        } else {
            None
        }
    }

    fn subtract(&self, other: &Self) -> Vec<Box<Self>> {
        if self.0.start >= other.0.end || self.0.end <= other.0.start {
            return vec![Box::new(self.clone())];
        }

        let mut difference = vec![];

        if self.0.start < other.0.start {
            difference.push(Box::new(Range(self.0.start..other.0.start)))
        }

        if self.0.end > other.0.end {
            difference.push(Box::new(Range(other.0.end..self.0.end)))
        }

        difference
    }
}
