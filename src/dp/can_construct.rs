use std::collections::HashMap;

pub fn can_construct_memoized(
    target: &str,
    word_bank: &[&str],
    memo: &mut HashMap<String, bool>,
) -> bool {
    if target.is_empty() {
        return true;
    }

    if word_bank.is_empty() {
        return false;
    }

    if memo.get(target).is_some() {
        return *memo.get(target).unwrap();
    }

    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap();

            if can_construct_memoized(suffix, word_bank, memo) {
                memo.insert(target.into(), true);
                return true;
            }
        }
    }

    memo.insert(target.into(), false);
    false
}

pub fn can_construct(target: &str, word_bank: &[&str]) -> bool {
    let mut memo = HashMap::new();
    can_construct_memoized(target, word_bank, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert!(can_construct("abcdef", &["ab", "abc", "cd", "def", "abcd"]));
        assert!(!can_construct(
            "skateboard",
            &["bo", "rd", "ate", "t", "ska", "sk", "boar"]
        ));
        assert!(can_construct(
            "eeeeee",
            &["e", "ee", "eee", "eeee", "eeeee"]
        ));
        assert!(!can_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &["e", "ee", "eee", "eeee", "eeeee"]
        ));
    }
}
