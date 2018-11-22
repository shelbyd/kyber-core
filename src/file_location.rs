use failure::Error;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileLocation {
    pub line: usize,
    pub column: usize,
}

impl FileLocation {
    pub fn new(line: usize, column: usize) -> FileLocation {
        FileLocation { line, column }
    }

    pub fn parse_1_indexed(line: &str, column: &str) -> Result<FileLocation, Error> {
        Ok(FileLocation::new(
            line.parse::<usize>()? - 1,
            column.parse::<usize>()? - 1,
        ))
    }

    pub fn index(&self, s: &str) -> Result<usize, Error> {
        let this_index = line_index(self.line, s)? + self.column;

        if let Some(index) = line_index(self.line + 1, s).ok() {
            ensure!(
                index > this_index,
                "Column {} out of range for line {}",
                self.column,
                self.line
            );
        }
        ensure!(
            this_index < s.len(),
            "Location {},{} out of file range",
            self.line,
            self.column
        );

        Ok(this_index)
    }
}

impl From<(usize, usize)> for FileLocation {
    fn from((line, column): (usize, usize)) -> FileLocation {
        FileLocation::new(line, column)
    }
}

pub fn get(text: &str, range: RangeInclusive<FileLocation>) -> Result<&str, Error> {
    let start_index = range.start().index(text)?;
    let last_index = range.end().index(text)?;
    Ok(&text[start_index..=last_index])
}

fn line_index(line: usize, text: &str) -> Result<usize, Error> {
    if line == 0 {
        Ok(0)
    } else {
        let nth_newline = text.match_indices("\n").skip(line - 1).next();
        Ok(nth_newline
            .ok_or(format_err!("Line {} out of range", line))?
            .0
            + 1)
    }
}

pub fn parse_range(s: &str) -> Result<(FileLocation, FileLocation), Error> {
    let regex = regex::Regex::new(r"(\d+),(\d+):(\d+),(\d+)")?;
    let captures = regex.captures(s).unwrap();

    let start = FileLocation::parse_1_indexed(&captures[1], &captures[2])?;
    let end = FileLocation::parse_1_indexed(&captures[3], &captures[4])?;

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod extract_range {
        use super::*;

        const TEXT: &'static str = "foo\nbar\nbaz";

        fn test_extract(range: RangeInclusive<(usize, usize)>) -> Result<&'static str, Error> {
            get(TEXT, (*range.start()).into()..=(*range.end()).into())
        }

        #[test]
        fn first_line() {
            assert_eq!(test_extract((0, 0)..=(0, 0)).unwrap(), "f");
            assert_eq!(test_extract((0, 0)..=(0, 2)).unwrap(), "foo");
        }

        #[test]
        fn multi_line_start_of_line() {
            assert_eq!(test_extract((0, 0)..=(1, 0)).unwrap(), "foo\nb");
        }

        #[test]
        fn second_line_start_of_line() {
            assert_eq!(test_extract((1, 0)..=(2, 0)).unwrap(), "bar\nb");
        }

        #[test]
        fn second_line_sub_word() {
            assert_eq!(test_extract((1, 0)..=(1, 2)).unwrap(), "bar");
        }

        #[test]
        fn cross_lines_and_words() {
            assert_eq!(test_extract((0, 1)..=(1, 1)).unwrap(), "oo\nba");
        }

        #[test]
        fn line_out_of_range() {
            assert!(test_extract((4, 0)..=(4, 0)).is_err());
        }

        #[test]
        fn col_out_of_range() {
            assert!(test_extract((0, 0)..=(0, 4)).is_err());
        }

        #[test]
        fn last_line_col_out_of_range() {
            assert!(test_extract((2, 0)..=(2, 4)).is_err());
        }
    }
}
