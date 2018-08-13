use inner_common::*;

pub fn bytes_lines<'a>(bytes: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    bytes.split(|&b| b == b'\n')
}

pub fn reflow(s: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let mut output = String::with_capacity(s.len() + s.len() / width);

    let mut x = 0;
    for word in s.split_whitespace() {
        x += word.len();

        if x > width {
            output.push('\n');

            x = word.len();
        } else if x > word.len() {
            output.push(' ');

            x += 1;
        }
        output.push_str(word);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_reflow_retains_all_non_whitespace() {
        quickcheck(reflow_retains_all_non_whitespace as fn((String, usize)) -> TestResult)
    }
    fn reflow_retains_all_non_whitespace((s, width): (String, usize)) -> TestResult {
        if width == 0 {
            return TestResult::discard();
        }

        let non_whitespace: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        let reflowed = reflow(&s, width);

        let reflowed_non_whitespace: String =
            reflowed.chars().filter(|c| !c.is_whitespace()).collect();

        assert_eq!(non_whitespace, reflowed_non_whitespace);

        TestResult::from_bool(non_whitespace == reflowed_non_whitespace)
    }

    #[test]
    fn max_length_words_reflow() {
        assert_eq!(
            reflow("1234567890123456789012345 1234567890123456789012345", 25),
            "1234567890123456789012345\n1234567890123456789012345".to_string()
        );
    }

    #[test]
    fn reflow_handles_word_split_at_exactly_the_len() {
        assert_eq!(
            reflow("CPU0, CPU1, CPU2, and you all win.", 25),
            "CPU0, CPU1, CPU2, and you\nall win.".to_string()
        );
    }
}
