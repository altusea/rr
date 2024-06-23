fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let sum = a.into_iter().filter(|e| e % 2 != 0).sum::<i32>();
    println!("{}", sum);

    let items = vec![1, 2, 3, 4, 5];
    let (even, odd): (Vec<i32>, Vec<i32>) = items.into_iter().partition(|&x| x % 2 == 0);
    println!("Even: {:?}", even); // 输出: Even: [2, 4]
    println!("Odd: {:?}", odd); // 输出: Odd: [1, 3, 5]
}
