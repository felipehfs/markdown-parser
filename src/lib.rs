enum State {
    Normal,
    Italic,
    Bold
}

fn parse(input: &'static str) -> String {
    use State::*;
    let mut initial_state = Normal;
    let mut output = String::new();

    for c in input.chars() {
        let (next_state, token) = match (initial_state, c) {
            (Normal, '*') => (Bold, "<b>".to_string()),
            (Normal, '_') => (Italic, "<em>".to_string()),
            (Normal, _) => (Normal, c.to_string()),
            (Bold, '*') => (Normal, "</b>".to_string()),
            (Bold, _) => (Bold, c.to_string()),
            (Italic, '_') => (Normal, "</em>".to_string()),
            (Italic, _) => (Italic, c.to_string())
        };

        token.chars().for_each(|x| output.push(x));
        initial_state = next_state;
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_bold() {
        assert_eq!(parse("*Hello*"), "<b>Hello</b>".to_string());
    }

    #[test]
    fn parse_italic() {
        assert_eq!(parse("_Hello_"), "<em>Hello</em>".to_string())
    }

    #[test]
    fn parse_normal() {
        assert_eq!(parse("Hello"), "Hello".to_string());
    }
}
