use std::ops::{Bound, RangeBounds};
use std::slice::SliceIndex;

pub trait ReplaceRange {
    fn replace_range<I, T: ToString>(&self, index: I, f: impl FnOnce(&str) -> T) -> Option<String>
    where
        I: SliceIndex<str, Output = str> + RangeBounds<usize>;
}

impl<S> ReplaceRange for S
where
    S: AsRef<str>,
{
    fn replace_range<I, T: ToString>(&self, index: I, f: impl FnOnce(&str) -> T) -> Option<String>
    where
        I: SliceIndex<str, Output = str> + RangeBounds<usize>,
    {
        let ref_ = self.as_ref();

        let start_string = match index.start_bound() {
            Bound::Unbounded => Some(""),
            Bound::Included(x) => ref_.get(..*x),
            Bound::Excluded(x) => ref_.get(..=*x),
        }?;
        let end_string = match index.end_bound() {
            Bound::Unbounded => Some(""),
            Bound::Included(x) => ref_.get((x + 1)..),
            Bound::Excluded(x) => ref_.get(*x..),
        }?;
        let middle_string = ref_.get(index)?;

        let new_string = String::from(start_string) + &f(middle_string).to_string() + end_string;
        Some(new_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_range_replaces_everything() {
        assert_eq!("foo".replace_range(.., |_| ""), Some("".to_string()));
    }

    #[test]
    fn full_range_calls_with_initial_string() {
        "foo".replace_range(.., |s| {
            assert_eq!(s, "foo");
            ""
        });
    }

    #[test]
    fn range_from_gives_from_range_in_argument() {
        "foo".replace_range(1.., |s| {
            assert_eq!(s, "oo");
            ""
        });
    }

    #[test]
    fn out_of_range_returns_none() {
        assert_eq!("foo".replace_range(4.., |_| ""), None);
    }

    #[test]
    fn range_from_keeps_outside_the_range() {
        assert_eq!("foo".replace_range(1.., |_| ""), Some("f".to_string()));
    }

    #[test]
    fn range_to_keeps_outside_the_range() {
        assert_eq!("foo".replace_range(..1, |_| ""), Some("oo".to_string()));
        assert_eq!("foo".replace_range(..2, |_| ""), Some("o".to_string()));
        assert_eq!("foo".replace_range(..3, |_| ""), Some("".to_string()));
    }
}
