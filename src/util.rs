use std::iter::Iterator;

pub fn range(start: i32, end: i32) -> Box<dyn Iterator<Item = i32>> {
    if start > end {
        return Box::new((end..=start).rev().into_iter());
    } else {
        return Box::new((start..=end).into_iter());
    }
}
