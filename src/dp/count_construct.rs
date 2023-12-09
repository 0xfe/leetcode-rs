use std::collections::HashMap;

pub fn count_construct_memoized(
    target: &str,
    word_bank: &[&str],
    memo: &mut HashMap<String, usize>,
) -> usize {
    if target.is_empty() {
        return 1;
    }

    if word_bank.is_empty() {
        return 0;
    }

    if memo.get(target).is_some() {
        return *memo.get(target).unwrap();
    }

    let mut total = 0;
    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap();
            total += count_construct_memoized(suffix, word_bank, memo);
        }
    }

    memo.insert(target.into(), total);
    total
}

pub fn count_construct(target: &str, word_bank: &[&str]) -> usize {
    let mut memo = HashMap::new();
    count_construct_memoized(target, word_bank, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(
            count_construct("purple", &["purp", "p", "ur", "le", "purpl"]),
            2
        );
        assert_eq!(
            count_construct("abcdef", &["ab", "abc", "cd", "def", "abcd"]),
            1
        );
        assert_eq!(
            count_construct("skateboard", &["bo", "rd", "ate", "t", "ska", "sk", "boar"]),
            0
        );
        assert_eq!(
            count_construct("eeeeee", &["e", "ee", "eee", "eeee", "eeeee", "eeeeee"]),
            32
        );
        assert_eq!(
            count_construct(
                "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
                &["e", "ee", "eee", "eeee", "eeeee"]
            ),
            0
        );
    }
}
