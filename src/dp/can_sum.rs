use std::collections::HashMap;

pub fn can_sum_memoized(target: i32, numbers: &[i32], memo: &mut HashMap<i32, bool>) -> bool {
    if target == 0 {
        return true;
    };

    if target < 0 {
        return false;
    };

    if memo.get(&target).is_some() {
        return *memo.get(&target).unwrap();
    }

    for &num in numbers {
        let remainder = target - num;

        if can_sum_memoized(remainder, numbers, memo) {
            return true;
        }
    }

    memo.insert(target, false);
    false
}

pub fn can_sum_iterative(target: i32, numbers: &[i32]) -> bool {
    let mut tab = vec![false; (target + 1) as usize];
    tab[0] = true;

    for i in 0..=target {
        if tab[i as usize] {
            for j in numbers {
                if i + j <= target {
                    tab[(i + j) as usize] = true;
                }

                if i + j == target {
                    return true;
                }
            }
        }
    }

    false
}

pub fn can_sum(target: i32, numbers: &[i32]) -> bool {
    let mut memo = HashMap::new();
    let a = can_sum_memoized(target, numbers, &mut memo);
    let b = can_sum_iterative(target, numbers);
    assert_eq!(a, b);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert!(can_sum(7, &[2, 3]));
        assert!(can_sum(7, &[5, 3, 4, 7]));
        assert!(!can_sum(7, &[2, 4]));
        assert!(can_sum(8, &[2, 3, 5]));
        assert!(!can_sum(300, &[7, 14]));
    }
}
