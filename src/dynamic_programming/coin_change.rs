use crate::pretty_print;

pub fn recursive_ways_to_make_sum(coins: &Vec<u32>, n: usize, sum: i128) -> u32 {
    if sum == 0 {
        return 1
    }

    if n == 0 || sum < 0 {
        return 0
    }

    recursive_ways_to_make_sum(coins, n, sum - coins[n - 1] as i128)
        + recursive_ways_to_make_sum(coins, n - 1, sum)
}

pub fn memoized_ways_to_make_sum(coins: &Vec<u32>, n: usize, sum: i128) -> u32 {
    let mut dp: Vec<Vec<u32>> = vec![vec![0; (sum + 1) as usize]; n + 1];
    _memoized_ways_to_make_sum(coins, n, sum, &mut dp);
}

pub fn _memoized_ways_to_make_sum(coins: &Vec<u32>, n: usize, sum: i128, dp: &mut Vec<Vec<u32>>) -> u32 {
    if sum == 0 {
        return 1
    }

    if sum < 0 || n == 0 {
        return 0
    }

    if dp[n - 1][sum as usize] != 0 {
        return dp[n - 1][sum as usize] as u32
    }

    dp[n - 1][sum as usize] = _memoized_ways_to_make_sum(coins, n, sum - coins[n - 1] as i128, dp)
        + _memoized_ways_to_make_sum(coins, n - 1, sum, dp);
    dp[n - 1][sum as usize]
}