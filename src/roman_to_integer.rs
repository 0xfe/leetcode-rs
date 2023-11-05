// https://leetcode.com/problems/roman-to-integer/
pub fn roman_to_int(s: String) -> i32 {
    let mut acc = 0; //accumulator
    let mut total = 0;

    for c in s.chars() {
        let val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            _ => 1000,
        };

        if val > acc {
            total += val - (acc * 2);
        } else {
            total += val;
        }

        acc = val;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0013() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
