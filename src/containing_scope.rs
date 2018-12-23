use std::ops::Range;

pub fn containing_scope(s: &str, input_range: Range<usize>) -> Range<usize> {
    let opening_brace = s[..input_range.start]
        .rmatch_indices(matching_braces('{', '}'))
        .map(|(i, _)| i + 1)
        .next()
        .unwrap_or(0);

    let closing_brace = s[input_range.end..]
        .match_indices(matching_braces('}', '{'))
        .map(|(i, _)| i + input_range.end)
        .next()
        .unwrap_or(s.len());

    opening_brace..closing_brace
}

fn matching_braces(target: char, inverse: char) -> impl FnMut(char) -> bool {
    let mut depth = 0;
    move |c: char| {
        if c == target && depth == 0 {
            return true;
        }

        let d_depth = if c == inverse {
            1
        } else if c == target {
            -1
        } else {
            0
        };
        depth += d_depth;

        false
    }
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

    #[test]
    #[ignore]
    fn with_closing_brace_inside() {
        assert_eq!(
            containing_scope_selected("{{let `f}oo` = 5;}"),
            "{let f}oo = 5;"
        );
    }

    #[test]
    #[ignore]
    fn with_opening_brace_inside() {
        assert_eq!(
            containing_scope_selected("{let `f{oo` = 5;}}"),
            "let f{oo = 5;}"
        );
    }
}
