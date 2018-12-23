use std::ops::Range;

pub fn containing_scope(s: &str, input_range: Range<usize>) -> Range<usize> {
    let mut scope_depth = 0;
    let start_index = s[..input_range.start]
        .rmatch_indices(|c| {
            if c == '}' {
                scope_depth += 1;
            } else if c == '{' {
                if scope_depth == 0 {
                    return true;
                }
                scope_depth -= 1;
            }
            false
        })
        .map(|(i, _)| i + 1)
        .next()
        .unwrap_or(0);

    let mut scope_depth = 0;
    let end_index = s[input_range.end..]
        .match_indices(|c| {
            if c == '{' {
                scope_depth += 1;
            } else if c == '}' {
                if scope_depth == 0 {
                    return true;
                }
                scope_depth -= 1;
            }
            false
        })
        .map(|(i, _)| i + input_range.end)
        .next()
        .unwrap_or(s.len());

    start_index..end_index
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selected_str::SelectedStr;

    fn containing_scope_selected(s: &str) -> String {
        let selected = &s.parse::<SelectedStr<_>>().unwrap();
        let scope = containing_scope(selected.as_str(), selected.range());
        let selected = selected.with_range(scope);
        selected.range_str().to_string()
    }

    #[test]
    fn whole_file() {
        assert_eq!(containing_scope_selected("let `foo` = 5;"), "let foo = 5;");
    }

    #[test]
    fn inside_single_brace() {
        assert_eq!(
            containing_scope_selected("{let `foo` = 5;}"),
            "let foo = 5;"
        );
    }

    #[test]
    fn with_brace_before() {
        assert_eq!(
            containing_scope_selected("{{}let `foo` = 5;}"),
            "{}let foo = 5;"
        );
    }

    #[test]
    fn with_brace_after() {
        assert_eq!(
            containing_scope_selected("{let `foo` = 5;{}}"),
            "let foo = 5;{}"
        );
    }

    #[test]
    fn with_outer_brace_before() {
        assert_eq!(
            containing_scope_selected("{}{let `foo` = 5;}"),
            "let foo = 5;"
        );
    }
}
