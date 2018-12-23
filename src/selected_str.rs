use std::ops::{Bound, Range, RangeBounds};
use std::str::FromStr;

pub struct SelectedStr<S: AsRef<str>> {
    s: S,
    range: Range<usize>,
}

impl<S: AsRef<str>> SelectedStr<S> {
    pub fn new<R: RangeBounds<usize>>(s: S, range: R) -> SelectedStr<S> {
        let start = match range.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => s + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Excluded(s) => *s,
            Bound::Included(s) => s + 1,
            Bound::Unbounded => s.as_ref().len(),
        };
        SelectedStr {
            s,
            range: start..end,
        }
    }

    pub fn range_str(&self) -> &str {
        &self.s.as_ref()[self.range.clone()]
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.s.as_ref()
    }

    pub fn with_range(&self, range: Range<usize>) -> SelectedStr<S>
    where
        S: Clone,
    {
        SelectedStr {
            range,
            s: self.s.clone(),
        }
    }
}

impl FromStr for SelectedStr<String> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let match_indices = s.match_indices('`').collect::<Vec<_>>();
        if match_indices.len() == 2 {
            let start = match_indices[0].0;
            let end = match_indices[1].0 - 1;
            Ok(SelectedStr {
                s: s.replace("`", "").to_string(),
                range: start..end,
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod parse {
        use super::*;

        #[test]
        fn errors_with_no_range() {
            let parse = "something".parse::<SelectedStr<String>>();
            assert!(parse.is_err());
        }

        #[test]
        fn starts_at_range() {
            let parse = "`s`omething".parse::<SelectedStr<String>>().unwrap();
            assert_eq!(parse.range_str(), "s");
        }

        #[test]
        fn errors_with_one_tick() {
            let parse = "`something".parse::<SelectedStr<String>>();
            assert!(parse.is_err());
        }

        #[test]
        fn errors_with_three_ticks() {
            let parse = "`s`o`mething".parse::<SelectedStr<String>>();
            assert!(parse.is_err());
        }

        #[test]
        fn includes_empty_range() {
            let parse = "``something".parse::<SelectedStr<String>>().unwrap();
            assert_eq!(parse.range_str(), "");
        }

        #[test]
        fn includes_range_in_middle_of_word() {
            let parse = "som`eth`ing".parse::<SelectedStr<String>>().unwrap();
            assert_eq!(parse.range_str(), "eth");
        }

        #[test]
        fn removes_ticks_from_string() {
            let parse = "som`eth`ing".parse::<SelectedStr<String>>().unwrap();
            assert_eq!(parse.as_str(), "something");
        }
    }

    #[cfg(test)]
    mod new {
        use super::*;

        #[test]
        fn from_range_bounds() {
            let selected_str = SelectedStr::new("foo", 0..1);
            assert_eq!(selected_str.range_str(), "f");
        }

        #[test]
        fn from_inclusive_range() {
            let selected_str = SelectedStr::new("foo", 0..=1);
            assert_eq!(selected_str.range_str(), "fo");
        }

        #[test]
        fn unbounded_start() {
            let selected_str = SelectedStr::new("foo", ..1);
            assert_eq!(selected_str.range_str(), "f");
        }

        #[test]
        fn unbounded_end() {
            let selected_str = SelectedStr::new("foo", ..);
            assert_eq!(selected_str.range_str(), "foo");
        }

        #[test]
        fn excluded_start() {
            let selected_str = SelectedStr::new("foo", (Bound::Excluded(0), Bound::Unbounded));
            assert_eq!(selected_str.range_str(), "oo");
        }
    }
}
