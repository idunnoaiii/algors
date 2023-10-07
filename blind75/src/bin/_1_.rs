use core::num;
use std::collections::HashMap;

fn main() {}

/**
 * time: O(n^2)
 * space: O(1)
 */
fn _two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    return Vec::new();
}


/**
 * time: O(n)
 * space: O(n)
 */
fn _two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut memory = HashMap::<i32, i32>::new();

    for (idx, item) in nums.iter().enumerate() {
        if let Some(pos_value) = memory.get(&(target - item)) {
            return vec![*pos_value, idx as i32];
        }
        memory.entry(*item).or_insert(idx as i32);
    }

    vec![]
}

fn _two_sum_two_pointer(nums: Vec<i32>, target: i32) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(
            _two_sum_naive([2, 7, 11, 15].to_vec(), 9),
            [0, 1].to_vec()
        );
        assert_eq!(_two_sum_naive([3, 2, 4].to_vec(), 6), [1, 2].to_vec());
        assert_eq!(_two_sum_naive([3, 3].to_vec(), 6), [0, 1].to_vec());
    }

    #[test]
    fn test_hashmap() {
        assert_eq!(
            _two_sum_hashmap([2, 7, 11, 15].to_vec(), 9),
            [0, 1].to_vec()
        );
        assert_eq!(_two_sum_hashmap([3, 2, 4].to_vec(), 6), [1, 2].to_vec());
        assert_eq!(_two_sum_hashmap([3, 3].to_vec(), 6), [0, 1].to_vec());
    }

    fn ex3() {
        assert_eq!(_two_sum_hashmap([3, 3].to_vec(), 6), [0, 1].to_vec());
    }
}
