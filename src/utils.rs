/// Highly reusable snippets of code


/// Remove excess whitespace from a hardcoded multiline string. These strings
/// may have an extra empty line at the top and bottom and tabs or spaces
/// preceding each line.
///
/// Example:
///
/// ```
/// let x = "
///     a
///     b
/// ";
/// assert_eq!("a\nb", remove_multiline_whitespace(x));
/// ```
///
#[allow(dead_code)]
pub fn remove_multiline_whitespace(source: &str) -> String {

    let lines: Vec<&str> = source.trim().lines().collect();
    let mut lines_stripped: Vec<&str> = Vec::new();

    for line in lines.iter() {
        let line_stripped = line.trim();
        if !line_stripped.is_empty() {
            lines_stripped.push(line_stripped);
        }
    }

    return lines_stripped.join("\n");

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {

        let x = "
            a
            b
        ";
        let y = "a\nb";
        let z = remove_multiline_whitespace(&x);

        assert_ne!(x, y);  // sanity check
        assert_eq!(y, z);

    }

}