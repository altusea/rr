#![allow(dead_code)]

use std::cell::Cell;

fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0; // 左指针
    let mut right = height.len() - 1; // 右指针
    let mut max_area = 0; // 最大面积

    while left < right {
        // 计算当前面积
        let width = (right - left) as i32;
        let current_height = height[left].min(height[right]);
        let current_area = width * current_height;

        // 更新最大面积
        max_area = max_area.max(current_area);

        // 移动较短的垂线
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut candidates = candidates;

    // 对候选数组进行排序，便于剪枝
    candidates.sort();

    // 辅助函数：递归回溯
    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            // 找到一个有效组合
            result.push(current.clone());
            return;
        }

        for i in start..candidates.len() {
            let candidate = candidates[i];

            // 剪枝：如果当前候选值大于剩余目标值，后续的值也不符合条件
            if candidate > target {
                break;
            }

            // 选择当前候选值
            current.push(candidate);

            // 递归调用，注意 start 参数仍为 i，允许重复选择当前数字
            backtrack(candidates, target - candidate, i, current, result);

            // 回溯：撤销选择
            current.pop();
        }
    }

    // 调用回溯函数
    let mut current = Vec::new();
    backtrack(&candidates, target, 0, &mut current, &mut result);

    result
}

fn lev_distance(a: &str, b: &str) -> usize {
    // cases which don't require further computation
    if a.is_empty() {
        return b.chars().count();
    } else if b.is_empty() {
        return a.chars().count();
    }

    let mut dcol: Vec<_> = (0..=b.len()).collect();
    let mut t_last = 0;

    for (i, sc) in a.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in b.chars().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = std::cmp::min(current, next);
                dcol[j + 1] = std::cmp::min(dcol[j + 1], dcol[j]) + 1;
            }
            current = next;
            t_last = j;
        }
    }
    dcol[t_last + 1]
}

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();

    // Create a DP table to store the minimum distances
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the first row and column
    for i in 0..=m {
        dp[i][0] = i as i32;
    }
    for j in 0..=n {
        dp[0][j] = j as i32;
    }

    // Fill the DP table
    for i in 1..=m {
        for j in 1..=n {
            if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
            }
        }
    }

    // Return the minimum distance
    dp[m][n]
}

fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_so_far = nums[0];
    let mut min_so_far = nums[0];
    let mut result = nums[0];

    for i in 1..nums.len() {
        let curr = nums[i];
        let temp_max = max_so_far;
        max_so_far = std::cmp::max(curr, std::cmp::max(temp_max * curr, min_so_far * curr));
        min_so_far = std::cmp::min(curr, std::cmp::min(temp_max * curr, min_so_far * curr));

        result = std::cmp::max(result, max_so_far);
    }

    result
}

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut non_zero_index = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, non_zero_index);
            non_zero_index += 1;
        }
    }
}

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
