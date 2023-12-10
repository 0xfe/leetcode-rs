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

    let mut best_solution: Option<Vec<i32>> = None;
    for &num in numbers {
        let remainder = target - num;

        if let Some(solution) = best_sum_memoized(remainder, numbers, memo) {
            let mut solution = solution;
            solution.push(num);
            if best_solution.is_none() || solution.len() < best_solution.as_ref().unwrap().len() {
                best_solution = Some(solution);
            }
        }
    }

    memo.insert(target, best_solution.clone());
    best_solution
}

pub fn best_sum_iterative(target: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    let mut tab = vec![None; (target + 1) as usize];
    tab[0] = Some(vec![]);

    for i in 0..=target {
        if tab[i as usize].is_some() {
            let mut best_solution = None as Option<Vec<i32>>;
            for j in numbers {
                let loc = (i + j) as usize;
                if i + j <= target {
                    if tab[loc].is_none() {
                        tab[loc] = tab[i as usize].clone();
                    }

                    if let Some(ref mut v) = tab[loc] {
                        v.push(*j);
                    }
                }

                if i + j == target
                    && (best_solution.is_none()
                        || tab[loc].as_ref().unwrap().len() < best_solution.as_ref().unwrap().len())
                {
                    best_solution = tab[loc].clone();
                }
            }

            if best_solution.is_some() {
                return best_solution;
            }
        }
    }

    None
}

pub fn best_sum(target: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    let mut memo = HashMap::new();
    best_sum_memoized(target, numbers, &mut memo)
    // best_sum_iterative(target, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(best_sum(7, &[5, 3, 4, 7]), Some(vec![7]));
        assert_eq!(best_sum(7, &[2, 3]), Some(vec![3, 2, 2]));
        assert_eq!(best_sum(7, &[2, 4]), None);
        assert_eq!(best_sum(8, &[2, 3, 5]), Some(vec![5, 3]));
        assert_eq!(best_sum(300, &[7, 14]), None);
    }
}
