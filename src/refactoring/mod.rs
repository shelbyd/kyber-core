use containing_scope::*;
use failure::Error;
use regex::*;
use replace_range::*;

pub fn inline_variable(contents: &str, start: usize, end: usize) -> Result<String, Error> {
    let variable_name = &contents[start..=end];
    let expression_matcher = Regex::new(&format!("let {} = (?P<expr>.+);", variable_name))?;
    let variable_matcher = Regex::new(&format!("\\b{}\\b", variable_name))?;

    let range = start..(end + 1);

    Ok(contents
        .replace_range(containing_scope(contents, range), |s| {
            let expression = &expression_matcher.captures(&s).unwrap()["expr"];
            let expression = format!("({})", expression);
            let without_expression = expression_matcher.replace(s, "").to_string();
            variable_matcher
                .replace_all(&without_expression, expression.as_str())
                .to_string()
        })
        .unwrap())
}
