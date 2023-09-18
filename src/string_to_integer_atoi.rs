// https://leetcode.com/problems/string-to-integer-atoi/

enum State {
    Start,
    Sign,
    Zero,
    Number,
    Escape,
}

pub fn my_atoi(s: String) -> i32 {
    println!("\ns: {}", s);
    use State::*;

    let mut state = Start;
    let mut sign: i32 = 1;
    let mut digits = vec![];

    for c in s.chars() {
        match state {
            Start => match c {
                ' ' => continue,
                '0' => state = Zero,
                '+' => state = Sign,
                '-' => {
                    sign = -1;
                    state = Sign;
                }
                '1'..='9' => {
                    digits.push(c);
                    state = Number;
                }
                _ => return 0,
            },
            Sign => match c {
                '0' => state = Zero,
                '1'..='9' => {
                    digits.push(c);
                    state = Number;
                }
                _ => return 0,
            },
            Zero => match c {
                '0' => continue,
                '1'..='9' => {
                    digits.push(c);
                    state = Number;
                }
                _ => break,
            },
            Number => match c {
                '0'..='9' => digits.push(c),
                _ => state = Escape,
            },
            Escape => break,
        }
    }

    if digits.len() > 10 {
        return if sign == 1 { i32::MAX } else { i32::MIN };
    }

    let mut result: i32 = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        result = result.saturating_add(sign.saturating_mul(
            (digit.to_digit(10).unwrap() as i32).saturating_mul(10i32.saturating_pow(i as u32)),
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(my_atoi("42".to_string()), 42);
        assert_eq!(my_atoi("   -42".to_string()), -42);
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(my_atoi("words and 987".to_string()), 0);
        assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(my_atoi("-2147483648".to_string()), -2147483648);
        assert_eq!(my_atoi("-6147483648".to_string()), -2147483648);
        assert_eq!(my_atoi("0000-91283472332".to_string()), 0);
        assert_eq!(my_atoi("  0000000000012345678".to_string()), 12345678);

        assert_eq!(my_atoi("".to_string()), 0);
        assert_eq!(my_atoi(" ".to_string()), 0);
        assert_eq!(my_atoi("  ".to_string()), 0);
        assert_eq!(my_atoi("  -  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1".to_string()), 0);
        assert_eq!(my_atoi("  -  1  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3".to_string()), 0);
    }
}
