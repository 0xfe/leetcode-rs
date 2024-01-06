// https://leetcode.com/problems/longest-common-prefix/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i = 0;

    if strs.is_empty() {
        return "".to_string();
    }

    'top: loop {
        let mut c = None;
        for s in strs.iter() {
            if i >= s.len() {
                break 'top;
            }

            if let Some(c) = c {
                if c != s.as_bytes()[i] {
                    break 'top;
                }
            } else {
                c = Some(s.as_bytes()[i]);
            }
        }

        i += 1;
    }

    strs[0][0..i].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "dog".to_string(),
                "dog".to_string()
            ]),
            "dog".to_string()
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "doge".to_string(),
                "dogs".to_string(),
                "dog".to_string()
            ]),
            "dog".to_string()
        );
    }
}
