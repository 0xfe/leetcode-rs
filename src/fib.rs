use std::collections::HashMap;

// Naive version of fib(...)
pub fn fib_naive(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }

    fib_naive(n - 1) + fib_naive(n - 2)
}

// Memoized recursive version of fib(...)
pub fn _fib_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }

    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let a = _fib_memoized(n - 1, memo);
    let b = _fib_memoized(n - 2, memo);

    memo.insert(n, a + b);
    *memo.get(&n).unwrap()
}

pub fn fib_memoized(n: u64) -> u64 {
    let mut memo = HashMap::new();
    _fib_memoized(n, &mut memo)
}

// Iterative version of fib(...)
pub fn fib_iterative(n: u64) -> u64 {
    let mut map = HashMap::new();

    map.insert(0, 0);
    map.insert(1, 1);

    for i in 2..=n {
        let a = map.get(&(i - 1)).unwrap();
        let b = map.get(&(i - 2)).unwrap();

        map.insert(i, a + b);
    }

    *map.get(&n).unwrap()
}

#[inline]
pub fn fib(n: u64) -> u64 {
    fib_iterative(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(39), 63245986);
        assert_eq!(fib(40), 102334155);
        assert_eq!(fib(50), 12586269025);
    }
}
