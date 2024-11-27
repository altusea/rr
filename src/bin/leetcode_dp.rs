pub fn is_match(s: String, p: String) -> bool {
    let chars: Vec<char> = p.chars().collect();
    let s_len = s.len();
    let p_len = p.len();
    let mut dp = Vec::<Vec<bool>>::with_capacity(s_len + 1);
    for _ in 0..=s_len {
        dp.push(vec![false; p_len + 1]);
    }
    dp[0][0] = true;

    for i in 0..=s_len {
        for j in 1..=p_len {
            if chars[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2];
                if matches(&s, &p, i, j - 1) {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if matches(&s, &p, i, j) {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[s_len][p_len]
}

fn matches(s: &str, p: &str, i: usize, j: usize) -> bool {
    if i == 0 {
        return false;
    }
    let p_chars: Vec<char> = p.chars().collect();
    if p_chars[j - 1] == '.' {
        return true;
    }

    let s_chars: Vec<char> = s.chars().collect();
    s_chars[i - 1] == p_chars[j - 1]
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut ans = 1_i64;
    let (mut x, mut y) = (n as i64, 1_i64);
    while y < m as i64 {
        ans = ans * x / y;
        x += 1_i64;
        y += 1_i64;
    }

    ans as i32
}

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let columns = obstacle_grid[0].len();
    let mut cache = vec![0; columns];

    cache[columns - 1] = 1;

    for row in obstacle_grid.into_iter().rev() {
        let mut prev = 0;

        for (cell, cache_item) in row.into_iter().zip(&mut cache).rev() {
            if cell == 0 {
                *cache_item += prev;
            } else {
                *cache_item = 0;
            }

            prev = *cache_item;
        }
    }

    cache[0]
}

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let len = triangle.len();
    let mut dp = vec![0; len];
    dp[0] = triangle[0][0];
    for i in 1..len {
        dp[i] = dp[i - 1] + triangle[i][i];
        let mut j = i - 1;
        while j > 0 {
            dp[j] = dp[j - 1].min(dp[j]) + triangle[i][j];
            j -= 1;
        }
        dp[0] += triangle[i][0];
    }
    let mut min_total = dp[0];
    for item in dp.iter().take(len).skip(1) {
        min_total = min_total.min(*item);
    }
    min_total
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let (mut buy1, mut buy2, mut sell1, mut sell2) = (-prices[0], -prices[0], 0, 0);
    for price in prices.iter().take(len) {
        buy1 = buy1.max(-price);
        sell1 = sell1.max(buy1 + price);
        buy2 = buy2.max(sell1 - price);
        sell2 = sell2.max(buy2 + price);
    }
    sell2
}

use std::cmp::min;

pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    let (mut p2, mut p3, mut p5) = (1, 1, 1);

    for i in 2..=n {
        let (num2, num3, num5) = (dp[p2] * 2, dp[p3] * 3, dp[p5] * 5);
        dp[i] = min(min(num2, num3), num5);
        if dp[i] == num2 {
            p2 += 1;
        }
        if dp[i] == num3 {
            p3 += 1;
        }
        if dp[i] == num5 {
            p5 += 1;
        }
    }

    dp[n]
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let max = amount + 1;
    let mut dp = vec![max; max as usize];
    dp[0] = 0;
    let len = coins.len();
    for i in 1..=amount {
        for j in 0..len {
            if coins[j] <= i {
                dp[i as usize] = dp[i as usize].min(dp[(i - coins[j]) as usize] + 1);
            }
        }
    }
    if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    }
}

pub fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = vec![0i32; n + 1];
    for num in 1..=n {
        result[num] = result[num >> 1] + ((num as i32) & 1);
    }

    result
}

pub fn tribonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let (mut t0, mut t1, mut t2) = (0, 1, 1);
            for _ in 3..=n {
                let t = t0;
                t0 = t1;
                t1 = t2;
                t2 = t + t0 + t1;
            }
            t2
        }
    }
}

pub fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut nums = vec![0; (n + 1) as usize];
    nums[1] = 1;
    for i in 2..=n {
        let flag = i % 2;
        let half = (i / 2) as usize;
        if flag == 1 {
            nums[i as usize] = nums[half] + nums[half + 1];
        } else {
            nums[i as usize] = nums[half];
        }
    }

    *nums.iter().max().unwrap()
}

pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let n = security.len();
    if (n as i32) < time {
        return vec![];
    }
    let mut left = vec![0i32; n];
    let mut right = vec![0i32; n];
    for i in 1..n {
        if security[i] <= security[i - 1] {
            left[i] = left[i - 1] + 1;
        }
        if security[n - i - 1] <= security[n - i] {
            right[n - i - 1] = right[n - i] + 1;
        }
    }
    let mut result = Vec::new();
    for i in (time as usize)..n - (time as usize) {
        if left[i] >= time && right[i] >= time {
            result.push(i as i32);
        }
    }
    result
}

fn main() {}
