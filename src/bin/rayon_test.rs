use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input
        .par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}

fn main() {
    let vec_with_initial_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b = sum_of_squares(&vec_with_initial_values);
    println!("{}", b);
}
