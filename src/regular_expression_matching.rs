// https://leetcode.com/problems/regular-expression-matching/

#[derive(Debug)]
enum Matcher {
    Char(char),
    ManyChar(char),
    Any,
    ManyAny,
}

struct Regex {
    pub matchers: Vec<Matcher>,
}

impl Regex {
    pub fn compile(p: &str) -> Option<Regex> {
        use Matcher::*;
        let mut regex = vec![];

        for rei in p.chars() {
            match rei {
                '.' => regex.push(Any),
                'a'..='z' => regex.push(Char(rei)),
                '*' => {
                    let last_matcher = regex.pop();
                    if let Some(last_matcher) = last_matcher {
                        match last_matcher {
                            Char(c) => regex.push(ManyChar(c)),
                            Any => regex.push(ManyAny),
                            ManyAny => regex.push(ManyAny),
                            ManyChar(c) => regex.push(ManyChar(c)),
                        }
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        Some(Regex { matchers: regex })
    }

    fn _is_match(s: &str, matchers: &[Matcher]) -> bool {
        use Matcher::*;

        if matchers.is_empty() {
            return s.is_empty();
        }

        if s.is_empty() {
            return matchers.iter().all(|m| matches!(m, ManyChar(_) | ManyAny));
        }

        match matchers[0] {
            Char(c) => c == s.chars().nth(0).unwrap() && Self::_is_match(&s[1..], &matchers[1..]),
            Any => !s.is_empty() && Self::_is_match(&s[1..], &matchers[1..]),
            ManyChar(c) => {
                let mut i = 0;
                while i <= s.len() {
                    let matched = Self::_is_match(&s[i..], &matchers[1..]);
                    if matched {
                        return true;
                    }

                    if i < s.len() && s.chars().nth(i).unwrap() != c {
                        return false;
                    }

                    i += 1
                }
                matchers.len() == 1
            }
            ManyAny => {
                let mut i = 0;
                while i <= s.len() {
                    let matched = Self::_is_match(&s[i..], &matchers[1..]);
                    if matched {
                        return true;
                    }

                    i += 1
                }
                matchers.len() == 1
            }
        }
    }

    fn is_match(&self, s: &str) -> bool {
        Self::_is_match(s, &self.matchers)
    }
}

pub fn is_match(s: String, p: String) -> bool {
    let regex = Regex::compile(&p).unwrap();
    regex.is_match(s.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(!is_match(
            "acaabbaccbbacaabbbb".to_string(),
            "a*.*b*.*a*aa*a*".to_string()
        ));
        assert!(is_match("a".to_string(), ".*".to_string()));
        assert!(is_match("a".to_string(), "a*a*".to_string()));
        assert!(is_match("a".to_string(), "..*".to_string()));
        assert!(is_match("a".to_string(), "a*.".to_string()));
        assert!(is_match(
            "abbabaaaaaaacaa".to_string(),
            "a*.*b.a.*c*b*a*c*".to_string()
        ));

        assert!(!is_match(
            "mississippi".to_string(),
            "mis*is*p*.".to_string()
        ));

        assert!(!is_match("aa".to_string(), "a".to_string()));
        assert!(is_match("aa".to_string(), "a*".to_string()));
        assert!(is_match("ab".to_string(), ".*".to_string()));
        assert!(is_match("aab".to_string(), "c*a*b".to_string()));
        assert!(!is_match("aabb".to_string(), "c*a*b".to_string()));
        assert!(is_match("a".to_string(), "a".to_string()));
        assert!(!is_match("a".to_string(), "b".to_string()));
        assert!(!is_match("a".to_string(), "bb".to_string()));
        assert!(!is_match("ab".to_string(), ".*c".to_string()));
        assert!(is_match("amara".to_string(), ".*a".to_string()));
        assert!(is_match("a".to_string(), ".*a".to_string()));
        assert!(is_match("a".to_string(), "a*a".to_string()));
        assert!(is_match("aa".to_string(), "a*a".to_string()));
        assert!(is_match("aaa".to_string(), "aa*".to_string()));
        assert!(is_match("aaa".to_string(), "a*a".to_string()));
        assert!(is_match("aaa".to_string(), "a*a*a".to_string()));
        assert!(is_match("aaa".to_string(), ".*a*a".to_string()));
        assert!(is_match("aaa".to_string(), "a*.*a".to_string()));
    }
}
