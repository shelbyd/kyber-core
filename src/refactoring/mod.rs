use containing_scope::*;
use failure::Error;
use replace_range::*;

pub fn inline_variable(contents: &str, start: usize, end: usize) -> Result<String, Error> {
    let variable_name = &contents[start..=end];
    let expression_matcher = regex::Regex::new(&format!("let {} = (?P<expr>.+);", variable_name))?;

    let range = start..(end + 1);

    Ok(contents
        .replace_range(containing_scope(contents, range), |s| {
            let expression = &expression_matcher.captures(&s).unwrap()["expr"];
            expression_matcher
                .replace(&s, "")
                .replace(variable_name, expression)
        })
        .unwrap())
}
