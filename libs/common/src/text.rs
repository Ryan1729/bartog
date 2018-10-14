use inner_common::RANK_SUIT_PAIR_LAYOUT_CHAR;

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

        if x == width && x == word.len() {
            output.push_str(word);
            continue;
        }

        if x >= width {
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

pub fn bytes_reflow(bytes: &[u8], width: usize) -> Vec<u8> {
    if width == 0 {
        return Vec::new();
    }
    test_log!(width);
    let mut output = Vec::with_capacity(bytes.len() + bytes.len() / width);

    let mut x = 0;
    for word in bytes_split_whitespace(bytes) {
        test_log!(word);
        x += word.len();
        test_log!(x);
        test_log!(output);
        if x == width && x == word.len() {
            output.extend(word.iter());
            continue;
        }

        if x >= width {
            output.push(b'\n');

            x = word.len();
        } else if x > word.len() {
            output.push(b' ');

            x += 1;
        }
        output.extend(word.iter());
    }

    output
}

pub fn slice_until_first_0<'a>(bytes: &'a [u8]) -> &'a [u8] {
    let mut usable_len = 255;

    for i in 0..bytes.len() {
        if bytes[i] == 0 {
            usable_len = i;
            break;
        }
    }

    if usable_len == 255 {
        bytes
    } else {
        &bytes[..usable_len]
    }
}

// NOTE This does not use a general purpose definition of whitespace.
// This should count a byte as whitespace iff it has all blank
// pixels in this game's font.
#[inline]
pub fn is_byte_whitespace(byte: u8) -> bool {
    let lower_half_byte = byte & 0b0111_1111;
    lower_half_byte < RANK_SUIT_PAIR_LAYOUT_CHAR || lower_half_byte == b' '
}

//See NOTE above.
pub fn bytes_split_whitespace<'a>(bytes: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    bytes
        .split(|&b| is_byte_whitespace(b))
        .filter(|word| word.len() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_bytes_reflow_then_lines_produces_lines_of_the_correct_length() {
        quickcheck(
            bytes_reflow_then_lines_produces_lines_of_the_correct_length
                as fn((Vec<u8>, usize)) -> TestResult,
        )
    }
    fn bytes_reflow_then_lines_produces_lines_of_the_correct_length(
        (s, width): (Vec<u8>, usize),
    ) -> TestResult {
        if width == 0 {
            return TestResult::discard();
        }
        let bytes = &s;

        if bytes.iter().cloned().all(is_byte_whitespace) {
            return TestResult::discard();
        }

        if bytes_split_whitespace(bytes).any(|w| w.len() > width) {
            return TestResult::discard();
        }

        let reflowed = bytes_reflow(bytes, width);
        for line in bytes_lines(&reflowed) {
            assert!(line.len() <= width);
        }

        TestResult::from_bool(true)
    }

    #[test]
    fn test_bytes_reflow_works_for_this_generated_case() {
        let s = vec![27, 0, 27, 0, 27, 0, 27];
        let width = 6;

        let reflowed = bytes_reflow(&s, width);
        if !reflowed.ends_with(&[b'\n', 27]) {
            test_println!("reflowed {:?}", reflowed);
        }
        assert!(reflowed.ends_with(&[b'\n', 27]));
    }
    #[test]
    fn test_bytes_reflow_works_for_this_real_case() {
        let s = vec![
            99, 112, 117, 32, 48, 32, 112, 108, 97, 121, 101, 100, 32, 97, 110, 32, 65, 99, 101,
            32, 111, 102, 32, 104, 101, 97, 114, 116, 115,
        ];
        let width = 28;

        let reflowed = bytes_reflow(&s, width);

        assert!(reflowed.ends_with(&[b'\n', 104, 101, 97, 114, 116, 115]));
    }

    #[test]
    fn test_is_byte_whitespace_works_on_upper_half_values() {
        assert!(is_byte_whitespace(128));
        assert!(is_byte_whitespace(128 + 1));
        assert!(is_byte_whitespace(128 + 32));
        assert!(!is_byte_whitespace(128 + 48));
    }

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

    #[test]
    fn bytes_reflow_handles_word_split_just_before_the_len() {
        assert_eq!(
            bytes_reflow(b"CPU 1 played a Queen of clubs.", 28),
            b"CPU 1 played a Queen of\nclubs."
        );
    }

    #[test]
    fn reflow_does_not_add_a_space_if_there_is_no_room() {
        assert_eq!(reflow("12345 67890", 5), "12345\n67890".to_string());
    }

    #[test]
    fn bytes_reflow_does_not_add_a_space_if_there_is_no_room() {
        assert_eq!(bytes_reflow(b"12345 67890", 5), b"12345\n67890");
    }
}
