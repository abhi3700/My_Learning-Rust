pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    println!("Original array: {:?}", &nums);
    let mut res = vec![0, 0];

    // check for single/first combination of 2 elements if they sum up to the target
    for (i, ei) in nums.iter().enumerate() {
        for (j, ej) in nums.iter().enumerate() {
            // left index < right index
            if (i < j) && (ei + ej == target) {
                res[0] = i as i32;
                res[1] = j.try_into().unwrap();
                // NOTE: both is correct: `...as i32` & `.try_into().unwrap()`
                // The second method says 'convert & unwrap to the type needed'
            }
        }
    }
    println!("Final indices: {:?}", res);

    return res;
}
