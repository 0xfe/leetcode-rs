use std::{collections::HashMap, vec};

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

pub fn how_sum_iterative(target: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    let mut tab = vec![None; (target + 1) as usize];
    tab[0] = Some(vec![]);

    for i in 0..=target {
        if tab[i as usize].is_some() {
            for j in numbers {
                let loc = (i + j) as usize;
                if i + j <= target {
                    if tab[loc].is_none() {
                        tab[loc] = tab[i as usize].clone();
                    }

                    if let Some(v) = &mut tab[loc] {
                        v.push(*j);
                    }
                }

                if i + j == target {
                    return tab[loc].clone();
                }
            }
        }
    }

    None
}

pub fn how_sum(target: i32, numbers: &[i32]) -> Option<Vec<i32>> {
    // how_sum_iterative(target, numbers)
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
