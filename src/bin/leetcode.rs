#![allow(dead_code)]

use std::cell::Cell;

fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
    // 辅助函数：判断 target 是否是 s 的子序列
    fn is_subsequence(s: &str, target: &str) -> bool {
        let mut s_iter = s.chars();
        for ch in target.chars() {
            // 如果找不到当前字符 ch，则返回 false
            if s_iter.find(|&c| c == ch).is_none() {
                return false;
            }
        }
        true
    }

    // 筛选出所有是 s 子序列的字符串
    let mut candidates: Vec<&String> = dictionary
        .iter()
        .filter(|word| is_subsequence(&s, word))
        .collect();

    // 按照长度降序、字典序升序排序
    candidates.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b) // 长度相同时按字典序升序
        } else {
            b.len().cmp(&a.len()) // 长度不同时按长度降序
        }
    });

    // 返回第一个候选字符串（如果有），否则返回空字符串
    candidates.first().map_or("".to_string(), |s| (*s).clone())
}

fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
    // 去重并排序
    let mut nums = nums;
    nums.sort_unstable();
    nums.dedup();

    let mut sum: i64 = 0; // 存储结果
    let mut count = 0; // 已找到的数字数量
    let mut current = 1; // 当前尝试的数字

    for &num in &nums {
        // 尝试填充 [current, num-1] 范围内的数字
        while current < num && count < k {
            sum += current as i64;
            count += 1;
            current += 1;
        }
        // 跳过当前数字 num
        current = num + 1;
    }

    // 如果还需要更多数字，继续填充
    while count < k {
        sum += current as i64;
        count += 1;
        current += 1;
    }

    sum
}

fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut rst = nums.clone();
    rst.sort();

    let slice = &mut rst[..];
    let slice_of_cells = Cell::from_mut(slice).as_slice_of_cells();

    for w in slice_of_cells.windows(2).step_by(2) {
        Cell::swap(&w[0], &w[1]);
    }
    rst
}

pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    // 如果数组为空，直接返回0
    if nums.is_empty() {
        return 0;
    }

    // k 用于记录唯一元素的个数和位置
    let mut k = 1;

    // 从第二个元素开始遍历
    for i in 1..nums.len() {
        // 如果当前元素大于前一个元素，说明是非重复元素
        if nums[i] > nums[i - 1] {
            // 将当前元素放到 k 位置
            nums[k] = nums[i];
            k += 1;
        }
    }

    // 返回唯一元素的个数
    k as i32
}

fn main() {
    println!("todo")
}
