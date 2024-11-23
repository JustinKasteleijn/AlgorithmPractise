pub fn print_2d_table_with_indexes<T>(table: &Vec<Vec<T>>)
where
    T: std::fmt::Debug,
{
    print!("     ");
    for col_idx in 0..table[0].len() {
        print!("{:4} ", col_idx);
    }
    println!();

    for (row_idx, row) in table.iter().enumerate() {
        print!("{:4} |", row_idx);
        for cell in row.iter() {
            print!("{:4?} ", cell);
        }
        println!();
    }
}