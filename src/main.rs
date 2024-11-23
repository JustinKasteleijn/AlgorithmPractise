mod dynamic_programming;
mod pretty_print;

fn main() {
    let coins: Vec<u32> = vec![2, 5, 3, 6];
    let sum: i128 = 10;
    println!("Result {}", dynamic_programming::coin_change::memoized_ways_to_make_sum(&coins, coins.len(), sum));
}

