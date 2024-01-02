/*

https://leetcode.com/problems/integer-to-roman/

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

*/

pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut result = Vec::with_capacity(20);

    while num >= 1000 {
        result.push('M');
        num -= 1000;
    }

    if num >= 900 {
        result.push('C');
        result.push('M');
        num -= 900;
    }

    if num >= 500 {
        result.push('D');
        num -= 500;
    }

    if num >= 400 {
        result.push('C');
        result.push('D');
        num -= 400;
    }

    while num >= 100 {
        result.push('C');
        num -= 100;
    }

    if num >= 90 {
        result.push('X');
        result.push('C');
        num -= 90;
    }

    if num >= 50 {
        result.push('L');
        num -= 50;
    }

    if num >= 40 {
        result.push('X');
        result.push('L');
        num -= 40;
    }

    while num >= 10 {
        result.push('X');
        num -= 10;
    }

    if num >= 9 {
        result.push('I');
        result.push('X');
        num -= 9;
    }

    if num >= 5 {
        result.push('V');
        num -= 5;
    }

    if num >= 4 {
        result.push('I');
        result.push('V');
        num -= 4;
    }

    while num > 0 {
        result.push('I');
        num -= 1;
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0012() {
        assert_eq!(int_to_roman(3), "III".to_string());
        assert_eq!(int_to_roman(4), "IV".to_string());
        assert_eq!(int_to_roman(9), "IX".to_string());
        assert_eq!(int_to_roman(58), "LVIII".to_string());
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    }
}
