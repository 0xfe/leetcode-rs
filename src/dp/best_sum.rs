use std::collections::HashMap;

pub fn best_sum_memoized(
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

    let mut solutions = vec![];
    for &num in numbers {
        let remainder = target - num;

        if let Some(solution) = best_sum_memoized(remainder, numbers, memo) {
            let mut solution = solution;
            solution.push(num);
            solutions.push(solution);
        }
    }

    if solutions.is_empty() {
        memo.insert(target, None);
        None
    } else {
        let best_solution = solutions.iter().min_by(|a, b| a.len().cmp(&b.len()));
        memo.insert(target, best_solution.cloned());
        best_solution.cloned()
    }
}

pub fn best_sum(target: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    let mut memo = HashMap::new();
    best_sum_memoized(target, numbers, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(best_sum(7, &[2, 3]), Some(vec![3, 2, 2]));
        assert_eq!(best_sum(7, &[5, 3, 4, 7]), Some(vec![7]));
        assert_eq!(best_sum(7, &[2, 4]), None);
        assert_eq!(best_sum(8, &[2, 3, 5]), Some(vec![5, 3]));
        assert_eq!(best_sum(300, &[7, 14]), None);
    }
}
