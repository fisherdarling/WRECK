// this is stolen from luter
pub mod alphabet_translator {
    #[inline(always)]
    pub fn hex_to_char(hex: &str) -> char {
        let numeric_code = u8::from_str_radix(&hex, 16).unwrap();
        numeric_code as char
    }

    pub fn char_to_hex(c: char) -> String {
        let mut temp = String::new();
        let n = c as u8;
        let mut hex_code = format!("{:X}", n);
        if hex_code.len() == 1 {
            hex_code.insert(0, '0');
        }
        temp.push('x');
        temp.push_str(&hex_code);
        temp
    }

    // TODO probably could do this with some sort of collect and map
    pub fn char_to_hex_a_string(input: &str) -> String {
        let mut temp = String::new();
        for c in input.chars() {
            temp.push_str(&char_to_hex(c))
        }
        temp
    }

    pub fn char_to_hex_if_whitespace(c: char) -> String {
        // TODO are there more whitspaces?
        if c.is_ascii_whitespace() || c == '\\' {
            return char_to_hex(c);
        }
        c.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::alphabet_translator::alphabet_translator;

    #[test]
    fn char_to_hex_tests() {
        assert_eq!("a", alphabet_translator::char_to_hex_if_whitespace('a'));
        assert_eq!("x20", alphabet_translator::char_to_hex_if_whitespace(' '));
        assert_eq!("x0A", alphabet_translator::char_to_hex_if_whitespace('\n'));
        assert_eq!("x5C", alphabet_translator::char_to_hex_if_whitespace('\\'));
        assert_eq!("k", alphabet_translator::char_to_hex_if_whitespace('k'));
    }
    #[test]
    fn trans_with_wacks() {
        assert_eq!(
            "x61x62x5Cx61x62",
            alphabet_translator::char_to_hex_a_string("ab\\ab")
        );
    }
    #[test]
    fn simple_str_trans() {
        assert_eq!("x61x62", alphabet_translator::char_to_hex_a_string("ab"));
    }

    #[test]
    fn a_char() {
        assert_eq!("x61", alphabet_translator::char_to_hex('a'));
    }

    #[test]
    fn space_char() {
        assert_eq!("x20", alphabet_translator::char_to_hex(' '));
    }

    #[test]
    fn newline_char() {
        assert_eq!("x0A", alphabet_translator::char_to_hex('\n'));
    }

    #[test]
    fn wack_char() {
        assert_eq!("x5C", alphabet_translator::char_to_hex('\\'));
    }
    #[test]
    fn a_hex() {
        assert_eq!('a', alphabet_translator::hex_to_char("61"));
    }

    #[test]
    fn space_hex() {
        assert_eq!(' ', alphabet_translator::hex_to_char("20"));
    }

    #[test]
    fn newline_hex() {
        assert_eq!('\n', alphabet_translator::hex_to_char("0A"));
    }

    #[test]
    fn wack_hex() {
        assert_eq!('\\', alphabet_translator::hex_to_char("5C"));
    }
}
