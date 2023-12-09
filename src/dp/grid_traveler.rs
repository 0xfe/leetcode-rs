pub fn grid_traveler(m: usize, n: usize) -> usize {
    if n == 0 || m == 0 {
        return 0;
    }

    if n == 1 && m == 1 {
        return 1;
    }

    let mut tab = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                tab[i][j] = 0;
            } else if i == 1 && j == 1 {
                tab[i][j] = 1;
            } else {
                tab[i][j] = tab[i - 1][j] + tab[i][j - 1];
            }
        }
    }

    tab[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(grid_traveler(1, 1), 1);
        assert_eq!(grid_traveler(2, 3), 3);
        assert_eq!(grid_traveler(3, 2), 3);
        assert_eq!(grid_traveler(3, 3), 6);
        assert_eq!(grid_traveler(18, 18), 2333606220);
    }
}
