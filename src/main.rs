mod dynamic_programming;
mod pretty_print;

fn main() {
    let items: Vec<dynamic_programming::knapsack::Item> = vec![
        dynamic_programming::knapsack::Item {
            value: 2,
            weight: 3,
        },
        dynamic_programming::knapsack::Item {
            value: 2,
            weight: 1,
        },
        dynamic_programming::knapsack::Item {
            value: 4,
            weight: 3,
        },
        dynamic_programming::knapsack::Item {
            value: 5,
            weight: 4,
        },
        dynamic_programming::knapsack::Item {
            value: 3,
            weight: 2,
        }
    ];
    const CAPACITY: u32 = 7;
println!("Result {}", dynamic_programming::knapsack::dp_knapsack(&items, CAPACITY));
}

