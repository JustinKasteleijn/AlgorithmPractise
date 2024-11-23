mod dynamic_programming;
mod pretty_print;

fn main() {
    let coins: Vec<u32> = (1..4).collect();
    let sum: i128 = 4;
    println!("Result {}", dynamic_programming::coin_change::memoized_ways_to_make_sum(&coins, coins.len(), sum));
}

