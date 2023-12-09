use std::collections::HashMap;

pub fn all_construct_memoized(
    target: &str,
    word_bank: &[&str],
    memo: &mut HashMap<String, Vec<Vec<String>>>,
) -> Vec<Vec<String>> {
    if target.is_empty() {
        return vec![vec![]];
    }

    if word_bank.is_empty() {
        return vec![];
    }

    if memo.get(target).is_some() {
        return memo.get(target).unwrap().clone();
    }

    let mut result = vec![];
    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap();
            let ways = all_construct_memoized(suffix, word_bank, memo);
            result.extend(ways.iter().map(|way| {
                let mut way = way.clone();
                way.insert(0, word.into());
                way
            }));
        }
    }

    memo.insert(target.into(), result.clone());
    result
}

pub fn all_construct(target: &str, word_bank: &[&str]) -> Vec<Vec<String>> {
    let mut memo = HashMap::new();
    all_construct_memoized(target, word_bank, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(
            all_construct("purple", &["purp", "p", "ur", "le", "purpl"]),
            vec![vec!["purp", "le"], vec!["p", "ur", "p", "le"]]
        );
        assert_eq!(
            all_construct("abcdef", &["ab", "abc", "cd", "def", "abcd", "ef", "c"]),
            vec![
                vec!["ab", "cd", "ef"],
                vec!["ab", "c", "def"],
                vec!["abc", "def"],
                vec!["abcd", "ef"]
            ]
        );
        assert!(
            all_construct("skateboard", &["bo", "rd", "ate", "t", "ska", "sk", "boar"]).is_empty()
        );
        assert_eq!(
            all_construct("eeeeee", &["e", "ee", "eee", "eeee", "eeeee", "eeeeee"]).len(),
            32
        );
        assert!(all_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &["e", "ee", "eee", "eeee", "eeeee"]
        )
        .is_empty());
    }
}
