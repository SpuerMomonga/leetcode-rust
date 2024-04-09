use std::collections::HashMap;

/// ## 1.两数之和
///
/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 你可以按任意顺序返回答案。
///
/// ### 示例 1：
///
/// > **输入：** `nums = [2,7,11,15], target = 9`  
/// > **输出：** `[0,1]`  
/// > **解释：** `因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。`  
///
/// ### 示例 2：
///
/// > **输入：** `nums** = [3,2,4], target = 6`  
/// > **输出：** `[1,2]`  
///
/// ### 示例 3：
///
/// > **输入：** `nums** = [3,3], target = 6`
///
/// ### 提示：
///
/// - `2 <= nums.length <= 10^4`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `-10^9 <= target <= 10^9`
/// - **只会存在一个有效答案**
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, el) in nums.into_iter().enumerate() {
            if let Some(&value) = map.get(&(target - el)) {
                return vec![value, i as i32];
            }
            map.insert(el, i as i32);
        }
        panic!()
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn case_1() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1])
    }

    #[test]
    fn case_2() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn case_3() {
        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1])
    }

    #[test]
    fn case_4() {
        let result = Solution::two_sum(vec![7, 1, 2, 3, 4, 5], 10);
        assert_eq!(result, vec![0, 3])
    }

    #[test]
    fn case_5() {
        let result = Solution::two_sum(vec![44, 56, 90, 67, 23, 63], 113);
        assert_eq!(result, vec![2, 4])
    }
}
