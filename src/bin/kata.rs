#![allow(dead_code)]

use std::collections::{BinaryHeap, HashSet};
use std::net::Ipv4Addr;

use itertools::Itertools;

fn name_shuffler(s: &str) -> String {
    s.split_whitespace().rev().join(" ")
}

fn get_k_smallest<T: Copy + Ord + PartialOrd>(arr: &mut [T], k: usize) -> Vec<T> {
    if arr.is_empty() || k == 0 {
        return vec![];
    }

    let k = k.min(arr.len());

    // Quickselect implementation
    fn quickselect<T: Copy + Ord + PartialOrd>(arr: &mut [T], k: usize) {
        if arr.len() <= 1 {
            return;
        }

        let pivot_idx = partition(arr);
        match pivot_idx.cmp(&k) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => quickselect(&mut arr[..pivot_idx], k),
            std::cmp::Ordering::Less => quickselect(&mut arr[pivot_idx + 1..], k - pivot_idx - 1),
        }
    }

    // Lomuto partition scheme
    fn partition<T: Copy + Ord + PartialOrd>(arr: &mut [T]) -> usize {
        let pivot_idx = arr.len() - 1;
        let mut i = 0;
        for j in 0..pivot_idx {
            if arr[j] <= arr[pivot_idx] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot_idx);
        i
    }

    quickselect(arr, k - 1);
    arr[..k].to_vec()
}

fn build_matches_table(num_teams: u32) -> Vec<Vec<(u32, u32)>> {
    if num_teams < 2 {
        return vec![];
    }

    let mut matches_table = Vec::new();
    let mut teams: Vec<u32> = (1..=num_teams).collect();

    // If odd number of teams, add a dummy team
    if num_teams % 2 != 0 {
        teams.push(0);
    }

    let num_rounds = teams.len() - 1;

    for _ in 0..num_rounds {
        let mut round_matches = Vec::new();

        // Pair teams
        for i in 0..teams.len() / 2 {
            let team1 = teams[i];
            let team2 = teams[teams.len() - 1 - i];

            // Skip if either team is the dummy team
            if team1 != 0 && team2 != 0 {
                round_matches.push((team1, team2));
            }
        }

        matches_table.push(round_matches);

        // Rotate teams (keep first team fixed)
        teams = vec![teams[0]]
            .into_iter()
            .chain(teams[2..].iter().cloned())
            .chain(vec![teams[1]].into_iter())
            .collect();
    }

    matches_table
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
        match &self {
            Cons::Null => vec![],
            Cons::Cons(head, tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
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
            }
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
