fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("{:?}", b);

    // create a boolean vec with a fixed size
    let n = 1001;
    let mut bool_vec = vec![true; n];
    // implement a prime seive algorithm
    for i in 2..n {
        if bool_vec[i] {
            for j in (i * i..n).step_by(i) {
                bool_vec[j] = false;
            }
        }
    }
    let primes: Vec<usize> = bool_vec
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect();
    println!("{:?}", primes);
}
