#![allow(dead_code)]

use std::collections::{BinaryHeap, HashSet};
use std::net::Ipv4Addr;

fn special_number(n: u64) -> String {
    let mut n = n;
    while n != 0 {
        if n % 10 > 5 {
            return "NOT!!".into();
        }
        n /= 10
    }
    "Special!!".into()
}

fn nth_smallest(nums: &[i32], pos: usize) -> i32 {
    // 创建数组的副本并排序
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort();

    // 返回第pos小的元素（pos从1开始，所以索引是pos-1）
    sorted_nums[pos - 1]
}

fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut counts = std::collections::HashMap::new();
    for c in sip.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    // order by count, then by char
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts
}

fn rental_car_cost(d: u32) -> u32 {
    if d >= 7 {
        d * 40 - 50
    } else if d >= 3 {
        d * 40 - 20
    } else {
        d * 40
    }
}

fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap_or(0)
}

fn gimme(input_array: [i32; 3]) -> usize {
    // Find the median by sorting a copy and taking the middle element
    let mut sorted = input_array.clone();
    sorted.sort();
    let median = sorted[1];

    // Find and return the index of the median in the original array
    input_array.iter().position(|&x| x == median).unwrap()
}

fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    while !is_solved(puzzle) {
        let mut made_progress = false;

        // Scan each cell in the 9x9 grid
        for row in 0..9 {
            for col in 0..9 {
                // If cell is empty (0)
                if puzzle[row][col] == 0 {
                    // Get all valid numbers for this position
                    let valid_nums = get_valid_numbers(puzzle, row, col);

                    // If there's exactly one valid number, fill it in
                    if valid_nums.len() == 1 {
                        puzzle[row][col] = valid_nums[0];
                        made_progress = true;
                    }
                }
            }
        }

        // If no progress was made in this iteration, the puzzle might be invalid
        // or require guessing (which isn't needed for "easy" puzzles per the requirement)
        if !made_progress {
            break;
        }
    }
}

// Check if puzzle is fully solved (no zeros remaining)
fn is_solved(puzzle: &[[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] == 0 {
                return false;
            }
        }
    }
    true
}

// Get valid numbers that can be placed at puzzle[row][col]
fn get_valid_numbers(puzzle: &[[u8; 9]; 9], row: usize, col: usize) -> Vec<u8> {
    let mut used = [false; 10]; // Index 0 unused, 1-9 for numbers

    // Check row
    for c in 0..9 {
        used[puzzle[row][c] as usize] = true;
    }

    // Check column
    for r in 0..9 {
        used[puzzle[r][col] as usize] = true;
    }

    // Check 3x3 box
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            used[puzzle[r][c] as usize] = true;
        }
    }

    // Collect all unused numbers from 1-9
    let mut valid = Vec::new();
    for num in 1..=9 {
        if !used[num as usize] {
            valid.push(num);
        }
    }
    valid
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    // Handle empty matrix
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let n = matrix.len();
    let mut result = Vec::with_capacity(n * n);
    let mut top = 0;
    let mut bottom = n - 1;
    let mut left = 0;
    let mut right = n - 1;

    while top <= bottom && left <= right {
        // Traverse right
        for j in left..=right {
            result.push(matrix[top][j]);
        }
        top += 1;

        // Traverse down
        for i in top..=bottom {
            result.push(matrix[i][right]);
        }
        if right > 0 {
            right -= 1;
        }

        if top <= bottom {
            // Traverse left
            for j in (left..=right).rev() {
                result.push(matrix[bottom][j]);
            }
            bottom -= 1;
        }

        if left <= right {
            // Traverse up
            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }

    result
}

fn zip_with<F, T, S, R>(f: F, a: &[T], b: &[S]) -> Vec<R>
where
    F: Fn(T, S) -> R,
    T: Copy,
    S: Copy,
{
    a.iter().zip(b.iter()).map(|(&x, &y)| f(x, y)).collect()
}

fn multiply(a: &str, b: &str) -> String {
    // 处理特殊情况
    if a == "0" || b == "0" {
        return "0".to_string();
    }

    // 反转字符串，方便计算
    let a: Vec<char> = a.chars().rev().collect();
    let b: Vec<char> = b.chars().rev().collect();

    // 初始化结果数组
    let mut result = vec![0; a.len() + b.len()];

    // 逐位相乘
    for i in 0..a.len() {
        for j in 0..b.len() {
            let digit_a = a[i].to_digit(10).unwrap();
            let digit_b = b[j].to_digit(10).unwrap();
            result[i + j] += digit_a * digit_b;
        }
    }

    // 处理进位
    for i in 0..result.len() - 1 {
        if result[i] >= 10 {
            result[i + 1] += result[i] / 10;
            result[i] %= 10;
        }
    }

    // 去除前导零
    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }

    // 反转结果并转换为字符串
    result.into_iter().rev().map(|d| d.to_string()).collect()
}

fn exp_sum(n: u64) -> u64 {
    if n == 0 {
        return 1; // 空分区
    }

    // 初始化动态规划表
    let mut dp = vec![vec![0; (n + 1) as usize]; (n + 1) as usize];

    // 初始条件
    for i in 0..=n {
        dp[0][i as usize] = 1; // dp[0][k] = 1
    }

    // 填充动态规划表
    for i in 1..=n {
        for j in 1..=n {
            if j > i {
                dp[i as usize][j as usize] = dp[i as usize][i as usize];
            } else {
                dp[i as usize][j as usize] =
                    dp[(i - j) as usize][j as usize] + dp[i as usize][(j - 1) as usize];
            }
        }
    }

    dp[n as usize][n as usize]
}

#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            Cons::Cons(head, tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            },
        }
    }
}

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        //provide a convenient method to convert any iterable to an algebraic list.
        let mut iter = it.into_iter();
        if let Some(head) = iter.next() {
            Cons::Cons(head, Box::new(Cons::from_iter(iter)))
        } else {
            Cons::Null
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        // return a new algebraic list containing only elements that satisfy the predicate function.
        match self {
            Cons::Null => Cons::Null,
            Cons::Cons(head, tail) => {
                if fun(head) {
                    Cons::Cons(head.clone(), Box::new(tail.filter(fun)))
                } else {
                    tail.filter(fun)
                }
            },
        }
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
    where
        F: Fn(T) -> S,
        S: Clone,
    {
        // return a new algebraic list containing all elements resulting from applying the mapper function to them.
        match self {
            Cons::Null => Cons::Null,
            Cons::Cons(head, tail) => Cons::Cons(fun(head.clone()), Box::new(tail.map(fun))),
        }
    }
}

fn dbl_linear(n: usize) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(std::cmp::Reverse(1));
    seen.insert(1);

    let mut current = 0;
    for _ in 0..=n {
        if let Some(std::cmp::Reverse(val)) = heap.pop() {
            current = val;
            let y = 2 * current + 1;
            let z = 3 * current + 1;

            if seen.insert(y) {
                heap.push(std::cmp::Reverse(y));
            }
            if seen.insert(z) {
                heap.push(std::cmp::Reverse(z));
            }
        }
    }

    current
}

fn ips_between(start: &str, end: &str) -> u32 {
    let start: u32 = start.parse::<Ipv4Addr>().unwrap().into();
    let end: u32 = end.parse::<Ipv4Addr>().unwrap().into();
    end - start
}

fn main() {
    println!("{}", dbl_linear(10)); // 输出应该是 22
}
