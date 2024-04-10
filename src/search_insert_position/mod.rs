/// ## 3. 无重复字符的最长子串
///
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 请必须使用时间复杂度为 `O(log n)` 的算法。
///
/// ### 示例 1：
///
/// > **输入:** `nums = [1,3,5,6], target = 5`
/// > **输出:** `2`
///
/// ### 示例 2：
///
/// > **输入:** `nums = [1,3,5,6], target = 2`
/// > **输出:** `1`
///
/// ### 示例 3：
///
/// > **输入:** `nums = [1,3,5,6], target = 7`
/// > **输出:** `4`
///
/// ### 提示：
///
/// - `1 <= nums.length <= 10^4`
/// - `-10^4 <= nums[i] <= 10^4`
/// - `nums` 为 无重复元素 的 升序 排列数组
/// - `-10^4 <= target <= 10^4`
pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_1() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_2() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_3() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_4() {
        let result = Solution::search_insert(vec![2, 3, 5, 6], 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_5() {
        let result = Solution::search_insert(vec![2, 3, 5, 6], 2);
        assert_eq!(result, 0);
    }
}
