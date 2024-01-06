// https://leetcode.com/problems/container-with-most-water/

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;

    let mut lp = 0;
    let mut rp = height.len() - 1;

    while lp < rp {
        let area = (rp - lp) as i32 * height[lp].min(height[rp]);

        if area > max_area {
            max_area = area;
        }

        if height[lp] < height[rp] {
            lp += 1;
        } else {
            rp -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn test_4() {
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(max_area(vec![1, 2, 4, 3]), 4);
    }

    #[test]
    fn test_6() {
        assert_eq!(
            max_area(vec![
                76, 155, 15, 188, 180, 154, 84, 34, 187, 142, 22, 5, 27, 183, 111, 128, 50, 58, 2,
                112, 179, 2, 100, 111, 115, 76, 134, 120, 118, 103, 31, 146, 58, 198, 134, 38, 104,
                170, 25, 92, 112, 199, 49, 140, 135, 160, 20, 185, 171, 23, 98, 150, 177, 198, 61,
                92, 26, 147, 164, 144, 51, 196, 42, 109, 194, 177, 100, 99, 99, 125, 143, 12, 76,
                192, 152, 11, 152, 124, 197, 123, 147, 95, 73, 124, 45, 86, 168, 24, 34, 133, 120,
                85, 81, 163, 146, 75, 92, 198, 126, 191
            ]),
            18048
        );
    }
}
