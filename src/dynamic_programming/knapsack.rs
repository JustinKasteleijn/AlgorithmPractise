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