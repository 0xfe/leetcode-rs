use std::collections::HashMap;

pub fn how_sum_memoized(
    target: i32,
    numbers: &[i32],
    memo: &mut HashMap<i32, Option<Vec<i32>>>,
) -> Option<Vec<i32>> {
    if target == 0 {
        return Some(vec![]);
    };

    if target < 0 {
        return None;
    };

    if memo.get(&target).is_some() {
        return memo.get(&target).unwrap().clone();
    }

    for &num in numbers {
        let remainder = target - num;

        if let Some(solution) = how_sum_memoized(remainder, numbers, memo) {
            let mut solution = solution;
            solution.push(num);
            return Some(solution);
        }
    }

    memo.insert(target, None);
    None
}

pub fn how_sum(target: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    let mut memo = HashMap::new();
    how_sum_memoized(target, numbers, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(how_sum(7, &[2, 3]), Some(vec![3, 2, 2]));
        assert_eq!(how_sum(7, &[5, 3, 4, 7]), Some(vec![4, 3]));
        assert_eq!(how_sum(7, &[2, 4]), None);
        assert_eq!(how_sum(8, &[2, 3, 5]), Some(vec![2, 2, 2, 2]));
        assert_eq!(how_sum(300, &[7, 14]), None);
    }
}
