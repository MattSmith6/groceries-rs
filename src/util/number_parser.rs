use regex::Regex;

pub struct NumParser {

    positive_integer_regex: Regex,
    positive_decimal_regex: Regex,

}

impl NumParser {

    pub fn new() -> NumParser {
        NumParser {
            positive_integer_regex: Regex::new("([0-9]+)").unwrap(),
            positive_decimal_regex: Regex::new("([0-9]+[.]?[0-9]+)").unwrap(),
        }
    }

    pub fn parse_positive_integer(&self, input: &str) -> Option<i32> {
        self.positive_integer_regex.find(input)
            .map(|m| m.as_str().parse::<i32>().unwrap())
    }

    pub fn parse_positive_decimal(&self, input: &str) -> Option<f32> {
        self.positive_decimal_regex.find(input)
            .map(|m| m.as_str().parse::<f32>().unwrap())
    }

}

impl Default for NumParser {

    fn default() -> NumParser {
        NumParser::new()
    }

}

#[cfg(test)]
mod tests {
    use crate::util::NumParser;

    #[test]
    fn new_no_panic_on_regex_initialization_unwraps() {
        NumParser::new();
    }

    #[test]
    fn parse_positive_integer() {
        assert_eq!(NumParser::new().parse_positive_integer("a 12345 b"), Some(12345_i32));
    }

    #[test]
    fn parse_positive_decimal_no_decimal_point() {
        assert_eq!(NumParser::new().parse_positive_decimal("a 34 g"), Some(34_f32));
    }

    #[test]
    fn parse_positive_decimal_with_decimal_point() {
        assert_eq!(NumParser::new().parse_positive_decimal("a 2.3 g"), Some(2.3_f32));
    }

}

