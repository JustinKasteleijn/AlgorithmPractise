pub fn recursive_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
}

pub fn memoization_fibonacci(n: usize) -> u128 {
    _memoization_fibonacci(n, &mut Vec::with_capacity(n + 1))
}

fn _memoization_fibonacci(n: usize, dp: &mut Vec<u128>) -> u128 {
    if n <= 1 {
        return  1
    }

    if let Some(&result) = dp.get(n) {
        return result;
    }

    let result = _memoization_fibonacci(n - 1, dp) + _memoization_fibonacci(n - 2, dp);
    dp.push(result);
    result
}

pub fn bottom_up_memoization_fibonacci(n: usize) -> u128 {
    let dp: &mut Vec<u128> = &mut Vec::with_capacity(n + 1);
    dp.push(1);
    dp.push(1);

    for i in 2..=n {
          dp.push(dp[i-1] + dp[i-2]);
    }

    dp[n]
}