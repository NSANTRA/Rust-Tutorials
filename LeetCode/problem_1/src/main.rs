#[warn(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len()-1 {
        let a = target - &nums[i];

        for j in i+1..nums.len() {
            if a == nums[j] {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1 () {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9))
    }

    #[test]
    fn case_2 () {
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6))
    }

    #[test]
    fn case_3 () {
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6))
    }
}