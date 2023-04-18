/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    // 优雅的解决方案，时间复杂度小于 O(n2)，但是空间复杂度为 O(n)
    // Elegant solution it has a time complexity less than O(n2) and a space complexity of O(n)
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in numbers.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![i as i32, j as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }

    // 普通的解决方案，时间复杂度为 O(n2)，空间复杂度为 O(1)
    // common‘s solution it has a time complexity of O(n2) and a space complexity of O(1)
    pub fn two_sum_commons(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        for idx_a in 0..numbers.len() {
            let mut idx_b = idx_a + 1;
            while idx_b < numbers.len() {
                if numbers[idx_a] + numbers[idx_b] == target {
                    ret.push(idx_a as i32);
                    ret.push(idx_b as i32);
                    return ret
                }
                idx_b += 1;
            }
        }
        ret
    }
}
// @lc code=end

