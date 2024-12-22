use crate::pretty_print;
pub struct Item {
    pub value: u32,
    pub weight: u32,
}

pub fn recursive_knapsack(items: &Vec<Item>, capacity: u32) -> u32 {
    _recursive_knapsack(items, capacity, items.len())
}


fn _recursive_knapsack(items: &Vec<Item>, capacity: u32, i: usize) -> u32 {
    if capacity == 0 || i == 0 {
        return 0
    }

    let item = &items[i-1];

    if item.weight > capacity {
        _recursive_knapsack(items, capacity, i-1)
    } else {
        std::cmp::max(
            item.value + _recursive_knapsack(items, capacity - item.weight, i-1),
            _recursive_knapsack(items, capacity, i-1)
        )
    }
}

pub fn dp_knapsack(items: &Vec<Item>, capacity: u32) -> u32 {
    let mut dp: Vec<Vec<u32>> = vec![vec![0; capacity as usize+1]; items.len()+1] ;

    for i in 1..=items.len() {
        let item = &items[i-1];
        for c in 1..=capacity as usize {
            dp[i][c] = dp[i-1][c];

            if item.weight <= c as u32 && dp[i-1][(c as u32 - item.weight) as usize] + item.value > dp[i][c] {
                dp[i][c] = dp[i-1][(c as u32 - item.weight) as usize] + item.value;
            }
        }
    }


    pretty_print::utils::print_2d_table_with_indexes(&dp);
    dp[items.len()][capacity as usize]
}