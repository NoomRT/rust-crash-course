pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    if nums.len() != 0 {
        for n in nums {
            sum += n;
        }
    } else {
        return 0;
    }
    sum
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v = vec![];
    for _ in 0..n {
        v.push(i);
    }
    v
}
