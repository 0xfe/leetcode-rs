// https://leetcode.com/problems/palindrome-number/

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    let mut started = false;
    let mut j = 0;

    for i in (0..10).rev() {
        let digit = x / 10i32.pow(i) % 10;

        if digit != 0 && !started {
            started = true;
            j = i;
        } else if !started {
            continue;
        }

        if digit != x / 10i32.pow(j - i) % 10 {
            return false;
        }

        if i < j / 2 {
            break;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0009() {
        assert!(is_palindrome(1221));
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
        assert!(!is_palindrome(1321));
        assert!(is_palindrome(113212311));
        assert!(is_palindrome(111111));
        assert!(!is_palindrome(111211));
    }
}
